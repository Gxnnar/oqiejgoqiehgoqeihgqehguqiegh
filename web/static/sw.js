const HOSTNAME = "http://localhost:8080";
const ABSOLUTE_URL_REGEX = new RegExp("^(?:[a-z+]+:)?//", "i");

self.addEventListener("fetch", (event) => {
  console.log("====================================");
  console.log(event.request);

  let url = new URL(event.request.url);

  // check if url is absolute
  // if (ABSOLUTE_URL_REGEX.test(url.href)) {
  //   event.respondWith(fetch(`/p/${encodeURIComponent(url.href)}`));
  //   return;
  // }

  let realHost = event.request.referrer.split("/p/")[1] ?? url;
  realHost = decodeURIComponent(realHost);
  realHost = new URL(realHost).origin;

  let req = new Request(
    `/p/${encodeURIComponent(realHost + url.pathname)}`,
    event.request
  );
  console.log(req);
  event.respondWith(fetch(req));
});
