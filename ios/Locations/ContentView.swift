//
//  ContentView.swift
//  Locations
//
//  Created by Sasha Weiss on 12/11/20.
//  Copyright © 2020 natasha-codes. All rights reserved.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        Button(action: {
            print("Hello world")
        }, label: {
            Text("Do the thing")
        })
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
