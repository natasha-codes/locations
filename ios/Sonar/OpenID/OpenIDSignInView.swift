//
//  OpenIDSignInView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/19/20.
//

import SwiftUI

struct OpenIDSignInView: View {
    let authority: OpenIDAuthority
    @ObservedObject var authSession: OpenIDAuthSession

    init(authority: OpenIDAuthority, authSession: OpenIDAuthSession) {
        self.authority = authority
        self.authSession = authSession
    }

    var body: some View {
        OpenIDView(buttonPrompt: "Sign in with \(self.authority.friendlyName)") { viewController in
            self.authSession.doSignIn(presenter: viewController) { result in
                switch result {
                case .success:
                    print("Sign in successful!")
                case let .failure(error):
                    print("Sign in failed: \(error)")
                }
            }
        }
    }
}
