//
//  SignedInView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import SwiftUI

struct SignedInView: View {
    @EnvironmentObject var authSession: OpenIDAuthSession

    var body: some View {
        Button("Get a token") {
            self.authSession.doWithAuth { result in
                switch result {
                case let .failure(err):
                    print("signed in view action error: \(err)")
                case let .success(token):
                    print("signed in view action token: \(token)")
                }
            }
        }
    }
}
