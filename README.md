
# Gets information about a Segment

req https://www.stedi.com/edi/_next/data/DHsqvsHFhs5sNzdlmx6ax/x12-008040/segment/ISA.json

- `pageProps.appearsInReleases`: string[] - Which releases it appears in
- `pageProps.seg.elements`: Element[] - The elements of the segment
- `pageProps.seg.purpose`: string - The description of the segment
- `pageProps.seg.segment_id`: string - The code of the segment
- `pageProps.seg.segment_name`: string - The name of the segment

Alternatively:

https://www.stedi.com/edi/x12-008040/segment/ISA

is an HTML page, and the JSON blob is embedded in the HTML contains the same information.
### Element

### Codes

https://www.stedi.com/edi/api/x12/008010/codes/I01

Returns array of these objects:

```json
{
"code_value": "00",
"content": "No Authorization Information Present (No Meaningful Information in I02)",
"paragraph_number": "1"
}
  ```


# Overview of Stedi file format

https://www.stedi.com/blog/getting-started-with-the-x12-file-format

