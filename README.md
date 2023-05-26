# SKURCode
This is a companion program to my Inventory Server (either Python or Rust versions)
written in Rust using Dioxus. The goal is to be ported to mobile, but the
current version is for PC.

## Setup
At the moment, since I can't get Dioxus to cooperate on dynamically setting
the ```onclick: move |_| ...``` colsure to the SKU of the product, these products
are all hardcoded in at the moment. To make personal use of this program, replace
the product images in /products and follow suite in hardcoding each image and SKU
onclick event. Ideally it should be doable, Dioxus has several examples that seem
to point in the right direction. Closest being dioxus/examples/calculator.rs,
however that code works because ```{k}``` is an int. Being a String causes copy
issue.

## Usage
As a companion program/app, the output goal is to create QR code based action
codes for Inventory Server. Products images in /products should be named with the
same SKUs as you have registered to Inventory Server.

### Buttons
The product images are buttons, and there are another 6 buttons bellow in the
action tray.
 - Product image: Select a SKU
 - Sell: Reduce quantity counter
 - Stock: Increase quantity counter
 - SKU*#: Generate the QR code of the current text in this field
 - Inspect: Used to check stock/sold quantiies stored by Inventory Server
 - Retire: Set SKU as being retired
 - Restore: Set SKU as no longer being retired
 - While in QR display, click to close the overlay

 ## WIP State:
 - Currently loads images based on cargo directory, the /products folder needs to
   be in the same directory as the .exe to function
 - Styling is bare-bones while focus on functionality is being completed
 - Some finer functionality that isn't batteries-included with Dioxus compared
   to the Python/Kivy version need working in
 - Mobile support is an end goal.
