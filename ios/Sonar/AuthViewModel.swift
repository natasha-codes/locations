//
//  AuthViewModel.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import AppAuth
import Combine
import Foundation
import SwiftUI

class AuthViewModel: ObservableObject {
    @Published var auth: Auth? = nil
}

struct Auth {
    typealias Token = String

    private let authState: OIDAuthState

    init(authState: OIDAuthState) {
        self.authState = authState
    }

    func doWithAuth(action: @escaping (_ token: Result<Token, AuthError>) -> Void) {
        self.authState.performAction { _accessToken, idToken, err in
            if let idToken = idToken {
                action(.success(idToken))
            } else if let err = err, let oidError = OIDErrorCode(rawValue: (err as NSError).code) {
                action(.failure(.openid(code: oidError)))
            } else {
                action(.failure(.unknown))
            }
        }
    }
}

enum AuthError: Error {
    case unknown
    case openid(code: OIDErrorCode)
}
