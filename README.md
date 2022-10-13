> **Warning**: Meetup no longer uses Swift in our codebase and will no longer maintain this.

# firestoker 🔥

firestoker automatically generates a client class for your api using a open api schema.

## usage 🪄

```bash
firestoker [OPTIONS] --url <URL>

Options:
  -u, --url <URL>        URL of the OpenAPI specification
  -o, --output <OUTPUT>  Output directory [default: ./]
  -f, --file <FILE>      [default: apiClient.swift]
  -h, --help             Print help information
  -V, --version          Print version information
```

## example 📝

```bash
firestoker --url http://localhost:3000/api-json

---
firestoker 🔥 | OpenAPI to Swift client generator

    - OpenAPI specification: http://localhost:3000/api-json
    - Output directory: ./
    - Output file: apiClient.swift
---
Fetching OpenAPI specification...
Parsing OpenAPI specification...
--- Generating Swift client ---
Generated function for path: /users/{uid}
Generated function for path: /users/{uid}/avatar
Generated function for path: /auth/setup
--- Finished generating Swift client ---
Writing to file...
Successfully ran swiftformat on the generated file
Done!
```

```swift
import Alamofire
import Foundation

class ApiClient {
    let token: String

    init(token: String) {
        self.token = token
    }

    func UsersController_getUser(uid: string) async {
        AF.request("\(Env().apiRoot)/users/\(uid)", method: .get, headers: ["Authorization": token]).responseDecodable(of: [ /** INSERT_PAYLOAD_HERE_GEN_PAYLOAD_NOT_IMPL */ ].self) { response in
            switch response.result {
            case let .success(value):
                return value
            case let .failure(error):
                throw error
            }
        }
    }

    func UsersController_updateUser(uid: string) async {
        AF.request("\(Env().apiRoot)/users/\(uid)", method: .patch, headers: ["Authorization": token]).responseDecodable(of: [ /** INSERT_PAYLOAD_HERE_GEN_PAYLOAD_NOT_IMPL */ ].self) { response in
            switch response.result {
            case let .success(value):
                return value
            case let .failure(error):
                throw error
            }
        }
    }

    func UsersController_setProfilePicture(uid: string) async {
        AF.request("\(Env().apiRoot)/users/\(uid)/avatar", method: .post, headers: ["Authorization": token]).responseDecodable(of: [ /** INSERT_PAYLOAD_HERE_GEN_PAYLOAD_NOT_IMPL */ ].self) { response in
            switch response.result {
            case let .success(value):
                return value
            case let .failure(error):
                throw error
            }
        }
    }

    func AuthController_setup() async {
        AF.request("\(Env().apiRoot)/auth/setup", method: .post, headers: ["Authorization": token]).responseDecodable(of: [ /** INSERT_PAYLOAD_HERE_GEN_PAYLOAD_NOT_IMPL */ ].self) { response in
            switch response.result {
            case let .success(value):
                return value
            case let .failure(error):
                throw error
            }
        }
    }
}
```

---
