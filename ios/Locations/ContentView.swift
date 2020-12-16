//
//  ContentView.swift
//  Locations
//
//  Created by Sasha Weiss on 12/11/20.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        Button(action: {
            discoverOpenID()
        }, label: {Text("Do the thing")})
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
