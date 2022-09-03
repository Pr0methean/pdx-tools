---
sidebar_label: API
---

# PDX Tools API

Before getting started with the PDX Tools API, you'll want to ensure you
have a PDX Tools account so you can get an API key. You can always
generate a new API key on the [account page](pathname:///account).

PDX Tools' philosophy is to have an overly simple API and push most of
the analysis client side so all uploaded saves can seamlessly take
advantage of new features as they are added to the analysis engine. If
it is desired to upload a save and then perform queries against the
save, then consider Skanderbeg's API.

Please use the [Discord](https://discord.gg/rCpNWQW) for help
with the API or if you have any suggestions.

## Local Analysis

It is not required to upload a save in order for it to be analyzed. PDX Tools is
able to analyze saves that are transferred to it within the browser. This means
that you could have a web page, which when visited, loads the save, opens the
[save analysis page](/), and then posts data to PDX Tools with the help of
[`Window.postMessage()`](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)

When PDX Tools has finished initializing it will send a message to your
tab with the contents of "pdx-tools-loaded", informing that PDX Tools is
now ready to receive the file.

You can refer to how [Pdx-Unlimiter](https://github.com/crschnick/pdx_unlimiter/blob/938af410955437f8232de61d4a4755f0357e01b8/resources/web/pdx_tools.html)
accomplishes it for an idea for your own implementation, but in short
the code looks something like:

```html
<html>
<body>
  <input type="file" />

  <script>
    (function () {
      function fileChange(e) {
        const file = e.currentTarget.files[0];
        const newWindow = window.open("https://pdx.tools/loading");

        window.addEventListener("message", (e) => {
          if (e.data === "pdx-tools-loaded") {
            newWindow.postMessage(file, "*");
          }
        });
      }

      document.querySelector("input").addEventListener("change", fileChange);
    })();
  </script>
</body>
</html>
```

## Upload

To upload a file to PDX Tools, it is easiest to run the [cli](https://github.com/rakaly/cli) which uses
this same endpoint and does all the heavy lifting. If an API is still
desired, below is the API endpoint:

```plain
POST https://pdx.tools/api/saves
```

All requests must be authenticated through [basic auth](https://en.wikipedia.org/wiki/Basic_access_authentication)
with your user id and API key.

All requests must contain the "rakaly-filename" HTTP header which
describes the filename of the uploaded files, as multipart/form-data is
not officially supported by the API.

The contents of the POST request will be the save. However, depending on
how the save is encoded, some additional steps must be taken. For saves
that are already a zip file, no additional steps are necessary and can
be uploaded. Saves that are not in a zip, must be first compressed in a
gzip-compatible format

First, an example request to upload a save that is a zip file.

```bash
curl "https://pdx.tools/api/saves" \
  --header "rakaly-filename: ita1.eu4" \
  --header "Content-Type: application/zip" \
  --data-binary @ita1.eu4 \
  --user "yourUserId"
```

An example request to upload save that is plaintext

```bash
gzip < ita1.eu4 | curl "https://pdx.tools/api/saves" \
  --header "rakaly-filename: ita1.eu4" \
  --header "Content-Type: text/plain; charset=windows-1252" \
  --header "Content-Encoding: gzip" \
  --data-binary @- \
  --user "yourUserId"
```

Setting the content type header to plain text is not required for the
request to be accepted but is recommended to be future proof.

Once uploaded, the server will return a response resembling:

```json
{
  "save_id": "xxx",
  "remaining_save_slots": 10,
  "used_save_slot": false
}
```

<details>
  <summary>Field breakdown</summary>
  <dl>
    <dt>save_id</dt>
    <dd>
      The unique identifier for the uploaded save. The save will be publicly
      hosted at /eu4/saves/xxx
    </dd>
    <dt>remaining_save_slots</dt>
    <dd>
      Informs the request how many save slots remain. By default all
      accounts have 100 save slots.
    </dd>
    <dt>used_save_slot</dt>
    <dd>
      Describes if the previous request consumed a save slot. Uploads
      consume a save slot if the save does not set a top 10 record for an
      achievement
    </dd>
  </dl>
</details>

If the file uploaded would consume a save slot but none are remaining,
then an HTTP 400 response is returned.

All API errors will return a JSON response that contains a user-friendly
"msg" field.

## Melt

Melting is the process of converting a binary encoded (eg: an ironman
save) into a plaintext save.

To melt a save offline, one can use the [cli](https://github.com/rakaly/cli) or interface with
[librakaly](https://github.com/rakaly/librakaly), which is a
shared library that one can integrate with any programming language.

There is no API endpoint for melting.
