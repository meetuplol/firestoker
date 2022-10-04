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
