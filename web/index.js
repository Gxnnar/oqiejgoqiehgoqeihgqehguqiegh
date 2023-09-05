const FORM = document.querySelector("[visit-site]");
const INPUT = FORM.querySelector("[url]");
const GO_BUTTON = FORM.querySelector("[go]");

// websites that the kool kids visit, presumably.
// i wouldn't know.
const PLACEHOLDER_SITES = [
  "https://connorcode.com",
  "https://google.com",
  "https://youtube.com",
  "https://github.com",
  "https://wikpedia.org",
  "https://chat.openai.com",
];
let placeholderIndex = 0;

GO_BUTTON.addEventListener("click", (e) => {
  e.preventDefault();
  const addr = INPUT.value;
  document.location = `/p/${encodeURIComponent(addr)}`;
});

function updatePlaceholder() {
  INPUT.placeholder = PLACEHOLDER_SITES[placeholderIndex];
  placeholderIndex = (placeholderIndex + 1) % PLACEHOLDER_SITES.length;
}

setInterval(updatePlaceholder, 2000);
updatePlaceholder();
