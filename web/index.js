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

const SUPPORTED_PROTOCOLS = ["http:", "https:"];

INPUT.addEventListener("keydown", () => {
  if (INPUT.value.length > 0) GO_BUTTON.removeAttribute("disabled");
  else GO_BUTTON.setAttribute("disabled", "");
});

GO_BUTTON.addEventListener("click", (e) => {
  let addr = INPUT.value;

  let hasProtocol = false;
  for (let protocol of SUPPORTED_PROTOCOLS) {
    if (addr.startsWith(protocol)) {
      hasProtocol = true;
      break;
    }
  }

  if (!hasProtocol) {
    addr = `https://${addr}`;
  }

  try {
    new URL(addr);
  } catch (e) {
    addr = `https://duckduckgo.com/?q=${encodeURIComponent(INPUT.value)}`;
  }

  document.location = `/~/${encodeURIComponent(addr)}`;
});

FORM.addEventListener("submit", (e) => {
  e.preventDefault();
  GO_BUTTON.click();
});

function updatePlaceholder() {
  INPUT.placeholder = `https://${PLACEHOLDER_SITES[placeholderIndex]}`;
  placeholderIndex = (placeholderIndex + 1) % PLACEHOLDER_SITES.length;
}

setInterval(updatePlaceholder, 2000);
updatePlaceholder();
