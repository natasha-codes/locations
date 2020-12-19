//
//  SignedInView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import SwiftUI

struct SignedInView: View {
    @EnvironmentObject var authSession: AuthSession

    var body: some View {
        Text("Signed in!")
            .onAppear() {
                self.authSession.doWithAuth { result in
                    switch result {
                    case let .failure(err):
                        print("\(err)")
                    case let .success(token):
                        print("token: \(token)")
                    }
                }
            }
    }
}
