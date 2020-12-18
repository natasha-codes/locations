//
//  SignedInView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import SwiftUI

struct SignedInView: View {
    private let auth: Auth

    init(auth: Auth) {
        self.auth = auth
    }

    var body: some View {
        Text("Signed in!")
            .onAppear() {
                self.auth.doWithAuth { result in
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
