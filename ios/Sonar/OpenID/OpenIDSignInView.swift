//
//  OpenIDSignInView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/19/20.
//

import SwiftUI

struct OpenIDSignInView: View {
    let authority: OpenIDAuthority

    @EnvironmentObject var authSession: OpenIDAuthSession

    init(authority: OpenIDAuthority) {
        self.authority = authority
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

struct OpenIDSignInView_Previews: PreviewProvider {
    static var previews: some View {
        OpenIDSignInView(authority: MSAOpenIDAuthority())
    }
}
