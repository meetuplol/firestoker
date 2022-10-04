use openapiv3::OpenAPI;

use clap::Parser;
use reqwest::blocking::get;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL of the OpenAPI specification
    #[arg(short, long)]
    url: String,

    /// Output directory
    #[arg(short, long, default_value = "./")]
    output: String,

    // Output file name
    #[arg(short, long, default_value = "apiClient.swift")]
    file: String,
}

fn generate_swift_func(path: String, data: openapiv3::PathItem) -> std::string::String {
    let mut functions = Vec::new();

    let methods = vec![
        ("get", data.get),
        ("put", data.put),
        ("post", data.post),
        ("delete", data.delete),
        ("options", data.options),
        ("head", data.head),
        ("patch", data.patch),
        ("trace", data.trace),
    ];

    for (method_name, method) in methods {
        if let Some(method) = method {
            let func_name = method.operation_id.unwrap();
            let func_params = path
                .split("/")
                .filter(|x| x.starts_with("{") && x.ends_with("}"))
                .map(|x| x.replace("{", "").replace("}", ": string"))
                .collect::<Vec<String>>()
                .join(", ");
            let func_path = path.replace("{", "\\(").replace("}", ")");

            let func_method = method_name;

            let func = format!(
                r#"
            func {}({}) async {{
                AF.request("\(Env().apiRoot){}", method: .{}, headers: ["Authorization": token]).responseDecodable(of: [ /** INSERT_PAYLOAD_HERE_GEN_PAYLOAD_NOT_IMPL */ ].self) {{ response in
                    switch response.result {{
                    case let .success(value):
                        return value
                    case let .failure(error):
                        throw error
                    }}
                }}
            }}
            "#,
                func_name, func_params, func_path, func_method
            );

            functions.push(func);
        }
    }

    return functions.join("\n");
}

fn create_swift_class(functions: String) -> std::string::String {
    return format!(
        r#"import Foundation
        import Alamofire

        class ApiClient {{
            let token: String

            init(token: String) {{
                self.token = token
            }}

            {}
        }}
        "#,
        functions
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!(
        r#"---
firestoker 🔥 | OpenAPI to Swift client generator

    - OpenAPI specification: {}
    - Output directory: {}
    - Output file: {}
---"#,
        args.url, args.output, args.file
    );

    println!("Fetching OpenAPI specification...");
    let resp = get(args.url)?.text()?;

    println!("Parsing OpenAPI specification...");
    let spec: OpenAPI = serde_json::from_str(&resp)?;

    println!("--- Generating Swift client ---");

    let mut functions = String::new();

    for (path, data) in spec.paths {
        let func = &generate_swift_func(path.clone(), data.as_item().unwrap().clone());
        functions.push_str(func);
        println!("Generated function for path: {}", path);
    }

    println!("--- Finished generating Swift client ---");

    let class = create_swift_class(functions);

    println!("Writing to file...");
    fs::write(format!("{}/{}", args.output, args.file), class)?;

    if let Ok(_) = std::process::Command::new("swiftformat")
        .arg(format!("{}/{}", args.output, args.file))
        .output()
    {
        println!("Successfully ran swiftformat on the generated file");
    } else {
        println!("Generated file is not formatted!");
    }

    println!("Done!");

    Ok(())
}
