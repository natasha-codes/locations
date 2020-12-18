//
//  MsaAuthView.swift
//  Sonar
//
//  Created by Sasha Weiss on 12/17/20.
//

import Foundation
import SwiftUI

struct MsaAuthView: View {
    var body: some View {
        OpenIDView<MSAOpenIDAuthority>()
    }
}
