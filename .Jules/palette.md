## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.
## 2026-07-06 - Error Page Accessibility Boundaries
**Learning:** The prompt explicitly prohibits complete page redesigns and new design dependencies (like custom fonts) even for error pages. Micro-UX updates should stay confined to minor CSS adjustments like focus states and basic ARIA attributes, adhering exactly to the original unstyled baseline.
**Action:** Always verify the original styling context first, and limit CSS/HTML additions strictly to what enhances accessibility (e.g., , ) without introducing new visual themes.
## 2024-07-06 - Error Page Accessibility Boundaries
**Learning:** The prompt explicitly prohibits complete page redesigns and new design dependencies (like custom fonts) even for error pages. Micro-UX updates should stay confined to minor CSS adjustments like focus states and basic ARIA attributes, adhering exactly to the original unstyled baseline.
**Action:** Always verify the original styling context first, and limit CSS/HTML additions strictly to what enhances accessibility (e.g., `role="alert"`, `:focus-visible`) without introducing new visual themes.
