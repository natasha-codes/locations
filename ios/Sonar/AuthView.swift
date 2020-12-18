//
//  AuthView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import Foundation
import SwiftUI

struct AuthView: View {
    @ObservedObject var authViewModel = AuthViewModel()

    var body: some View {
        if let _auth = self.authViewModel.auth {
            Text("You're signed in!")
        } else {
            OpenIDView<MSAOpenIDAuthority>()
        }
    }
}
