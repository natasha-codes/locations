//
//  OpenIDSignOutView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/19/20.
//

import SwiftUI

struct OpenIDSignOutView: View {
    let authority: OpenIDAuthority
    @ObservedObject var authSession: OpenIDAuthSession

    init(authority: OpenIDAuthority, authSession: OpenIDAuthSession) {
        self.authority = authority
        self.authSession = authSession
    }

    var body: some View {
        OpenIDView(buttonPrompt: "Sign out from \(self.authority.friendlyName)") { viewController in
            self.authSession.doSignOut(presenter: viewController) { result in
                switch result {
                case .success:
                    print("Sign out successful!")
                case let .failure(error):
                    print("Sign out failed: \(error)")
                }
            }
        }
    }
}
