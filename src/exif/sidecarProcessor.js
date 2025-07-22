import fs from 'fs';

/**
 * Sidecar file processing for RAW and unsupported formats
 */

export async function createSidecarFile(filePath, selection) {
  try {
    const sidecarPath = filePath + '.xmp';
    const xmpContent = generateXmpSidecar(selection);
    fs.writeFileSync(sidecarPath, xmpContent);
    console.log(`Created sidecar file: ${sidecarPath}`);
    return true;
  } catch (error) {
    console.error(`Error creating sidecar file for ${filePath}:`, error.message);
    return false;
  }
}

export function generateXmpSidecar(selection) {
  return `<?xml version="1.0" encoding="UTF-8"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/" x:xmptk="IFEX CLI">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description rdf:about=""
        xmlns:tiff="http://ns.adobe.com/tiff/1.0/"
        xmlns:exif="http://ns.adobe.com/exif/1.0/"
        xmlns:dc="http://purl.org/dc/elements/1.1/">
      <tiff:Make>${selection.setup.camera.maker}</tiff:Make>
      <tiff:Model>${selection.setup.camera.model}</tiff:Model>
      <exif:LensMake>${selection.setup.lens.maker}</exif:LensMake>
      <exif:LensModel>${selection.setup.lens.getLensModelWithAperture()}</exif:LensModel>
      <exif:ISOSpeedRatings>
        <rdf:Seq>
          <rdf:li>${selection.film.iso}</rdf:li>
        </rdf:Seq>
      </exif:ISOSpeedRatings>
      ${selection.shotISO ? `<exif:PhotographicSensitivity>${selection.shotISO}</exif:PhotographicSensitivity>` : ''}
      <dc:creator>
        <rdf:Seq>
          <rdf:li>${selection.photographer}</rdf:li>
        </rdf:Seq>
      </dc:creator>
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>`;
}

export function removeSidecarFile(filePath) {
  const sidecarPath = filePath + '.xmp';
  if (fs.existsSync(sidecarPath)) {
    fs.unlinkSync(sidecarPath);
    console.log(`Removed sidecar file: ${sidecarPath}`);
    return true;
  }
  return false;
}
