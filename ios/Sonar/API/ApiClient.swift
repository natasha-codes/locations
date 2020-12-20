//
//  ApiClient.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/19/20.
//

import Foundation
import SwiftUI

struct ApiClient {
    let authSession: OpenIDAuthSession

    func perform(action: @escaping (Result<Void, String>) -> Void) {
        self.authSession.doWithAuth { result in
            action(result.map { print("token: \($0)") }.mapError { "\($0)" })
        }
    }
}
