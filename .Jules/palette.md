## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2026-07-17 - Align Frontend Validation with Backend Rules
**Learning:** In server-rendered applications, generic error pages can cause users to lose their form input state. Adding HTML5 validation attributes (, , ) that strictly match the backend logic ensures users receive immediate, inline feedback, preventing frustration and state loss.
**Action:** Always verify that frontend form inputs include corresponding HTML5 validation for all backend constraints.

## $(date +%Y-%m-%d) - Align Frontend Validation with Backend Rules
**Learning:** In server-rendered applications, generic error pages can cause users to lose their form input state. Adding HTML5 validation attributes (`pattern`, `minlength`, `maxlength`) that strictly match the backend logic ensures users receive immediate, inline feedback, preventing frustration and state loss.
**Action:** Always verify that frontend form inputs include corresponding HTML5 validation for all backend constraints.

## 2024-07-28 - Visible Helper Text for Complex Form Validation
**Learning:** While `title` attributes can provide fallback validation hints, they aren't fully accessible and hide the constraints until a user attempts a submission. When inputs have complex `pattern` rules (e.g., custom alias restrictions), showing visible helper text linked via `aria-describedby` provides upfront instructions, reducing frustration for both sighted and screen-reader users.
**Action:** Pair complex validations (like `pattern` restrictions) with a visible helper text element and link them with `aria-describedby` to announce constraints proactively.
