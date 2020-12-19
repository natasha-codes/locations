//
//  APIClient.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/19/20.
//

import Foundation

class APIClient {
    private let authSession: OpenIDAuthSession

    init(authSession: OpenIDAuthSession) {
        self.authSession = authSession
    }
}
