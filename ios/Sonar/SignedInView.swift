//
//  SignedInView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/18/20.
//

import SwiftUI

struct SignedInView: View {
    let apiClient: ApiClient

    var body: some View {
        Button("Get a token") {
            self.apiClient.perform { result in
                switch result {
                case .success:
                    print("API client action success")
                case .failure(let err):
                    print("API client action error: \(err)")
                }
            }
        }
    }
}
