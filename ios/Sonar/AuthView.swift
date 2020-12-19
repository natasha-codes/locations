//
//  AuthView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import SwiftUI

struct AuthView: View {
    @ObservedObject var authSession = AuthSession()

    var body: some View {
        if self.authSession.hasAuthenticated {
            SignedInView()
                .environmentObject(self.authSession)
        } else {
            OpenIDView<MSAOpenIDAuthority>()
                .environmentObject(self.authSession)
        }
    }
}
