const FORM = document.querySelector("[visit-site]");
const INPUT = FORM.querySelector("[url]");
const GO_BUTTON = FORM.querySelector("[go]");

// websites that the kool kids visit, presumably.
// i wouldn't know.
let PLACEHOLDER_SITES = [
  "connorcode.com",
  "google.com",
  "youtube.com",
  "github.com",
  "wikpedia.org",
  "chat.openai.com",
];
let placeholderIndex = 0;

INPUT.addEventListener("keydown", () => {
  if (INPUT.value.length > 0) GO_BUTTON.removeAttribute("disabled");
  else GO_BUTTON.setAttribute("disabled", "");
});

GO_BUTTON.addEventListener("click", (e) => {
  e.preventDefault();
  const addr = INPUT.value;
  document.location = `/p/${encodeURIComponent(addr)}`;
});

function updatePlaceholder() {
  INPUT.placeholder = `https://${PLACEHOLDER_SITES[placeholderIndex]}`;
  placeholderIndex = (placeholderIndex + 1) % PLACEHOLDER_SITES.length;
}

setInterval(updatePlaceholder, 2000);
updatePlaceholder();
