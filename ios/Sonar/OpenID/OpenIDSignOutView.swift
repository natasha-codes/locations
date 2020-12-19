//
//  OpenIDSignOutView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/19/20.
//

import SwiftUI

struct OpenIDSignOutView<Authority: OpenIDAuthority>: View {
    @EnvironmentObject var authSession: OpenIDAuthSession<Authority>

    var body: some View {
        OpenIDView<Authority>(buttonPrompt: "Sign out from \(Authority.friendlyName)") { viewController in
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

struct OpenIDSignOutView_Previews: PreviewProvider {
    static var previews: some View {
        OpenIDSignOutView<MSAOpenIDAuthority>()
    }
}
