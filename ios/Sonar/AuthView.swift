//
//  AuthView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import SwiftUI

struct AuthView: View {
    @ObservedObject var authViewModel = AuthViewModel()

    var body: some View {
        if let auth = self.authViewModel.auth {
            SignedInView(auth: auth)
        } else {
            OpenIDView<MSAOpenIDAuthority>(auth: $authViewModel.auth)
        }
    }
}

struct AuthView_Previews: PreviewProvider {
    static var previews: some View {
        AuthView()
    }
}
