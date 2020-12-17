//
//  AuthView.swift
//  Locations
//
//  Created by Sasha Weiss on 12/11/20.
//

import SwiftUI

struct AuthView: View {
    var body: some View {
        let openIdView = OpenIDView()

        return openIdView
            .onOpenURL { [weak openIdView] url in
                guard let openIdView = openIdView else {
                    print("openIdView dealloced")
                    return
                }

                openIdView.handleAuthUrl(url: url)
            }
    }
}

struct AuthView_Previews: PreviewProvider {
    static var previews: some View {
        AuthView()
    }
}
