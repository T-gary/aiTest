feat: mobile preview mode with phone frame v0.3.0

- right toolbar "desktop version" renamed to "mobile version"
- clicking it renders the app inside a 390x844 device frame; layout switches via container queries
- Esc or the header "desktop version" button exits; state persisted in localStorage
- mobile home: full-width search row, horizontally scrollable categories, 160px banner, 2-column product grid
- checkout hides the unit price column at phone width so the table no longer overflows
- real devices (<=500px) drop the frame and fill the screen
