//
//  OpenIDSignOutView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/19/20.
//

import SwiftUI

struct OpenIDSignOutView: View {
    let authority: OpenIDAuthority

    @EnvironmentObject var authSession: OpenIDAuthSession

    init(authority: OpenIDAuthority) {
        self.authority = authority
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

struct OpenIDSignOutView_Previews: PreviewProvider {
    static var previews: some View {
        OpenIDSignOutView(authority: MSAOpenIDAuthority())
    }
}
