//
//  SignedInView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import SwiftUI

struct SignedInView: View {
    let apiClient: ApiClient

    @State private var tokenMessage: String = "Is there a token?"

    var body: some View {
        VStack(spacing: 48) {
            Button("Try and get a token") {
                self.apiClient.perform { result in
                    switch result {
                    case .success:
                        self.tokenMessage = "Yes!"
                    case let .failure(err):
                        self.tokenMessage = "Nope, error code: \(err)"
                    }
                }
            }
            Text(self.tokenMessage)
        }
    }
}
