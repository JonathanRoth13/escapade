const CANVAS_BACKGROUND_COLOR = "none";
const CANVAS_SCALE = 100;

const FILL_COLOR_LIGHT = "#D5B99E";
const FILL_COLOR_HOLLOW = "#000";
const SQUARE_PIECE_WIDTH = 3;
const SHORT_SQUARE_PIECE_HEIGHT = 4;

const COSMETIC_INDENT_HEIGHT = 1;
const COSMETIC_INDENT_LENGTH = 0.4;

const PIECE_START = -2;
const SHORT_PIECE_LENGTH_END = 2;

const CIRCLE_HOLE_RADIUS = 1;

// Calculate tight canvas size based on piece bounds
// For isometric projection, need to account for both horizontal and vertical projections
const PIECE_BOUNDS_WIDTH = SQUARE_PIECE_WIDTH * 1.8; // Isometric projection needs more horizontal space
const PIECE_BOUNDS_HEIGHT =
  SHORT_SQUARE_PIECE_HEIGHT + Math.abs(PIECE_START) + SQUARE_PIECE_WIDTH;
const CANVAS_WIDTH = PIECE_BOUNDS_WIDTH * CANVAS_SCALE + 200;
const CANVAS_HEIGHT = PIECE_BOUNDS_HEIGHT * CANVAS_SCALE + 200;

/* eslint-disable @typescript-eslint/no-require-imports */
const {
  IsometricCanvas,
  IsometricRectangle,
  IsometricCircle,
  IsometricPath,
  PlaneView,
} = require("@elchininet/isometric/node");

const fs = require("node:fs");
const path = require("node:path");
const sharp = require("sharp");
/* eslint-enable @typescript-eslint/no-require-imports */

// Create output directory
const outputDir = path.join(__dirname, "../public");
if (!fs.existsSync(outputDir)) {
  fs.mkdirSync(outputDir, { recursive: true });
}

// Helper function to save SVG
function saveSVG(filename, canvas) {
  const svgCode = canvas
    .getSVGCode()
    .replace("<svg ", '<svg xmlns="http://www.w3.org/2000/svg" ');
  fs.writeFileSync(path.join(outputDir, filename), svgCode);
  console.log(`Generated ${filename}`);
  return svgCode;
}

// Helper function to convert SVG to PNG
async function convertToPNG(svgCode, pngFilename, size = 512) {
  const buffer = Buffer.from(svgCode);
  await sharp(buffer)
    .resize(size, size)
    .png()
    .toFile(path.join(outputDir, pngFilename));
  console.log(`Generated ${pngFilename}`);
}

// Piece 7 (Short, Light, Hollow, Square) - Centered
const favicon_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});

// Calculate centering offset
const centerOffset = -SQUARE_PIECE_WIDTH / 2;

const favicon_top = new IsometricRectangle({
  height: SQUARE_PIECE_WIDTH,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_LIGHT,
});
const favicon_right = new IsometricRectangle({
  height: SHORT_SQUARE_PIECE_HEIGHT,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.FRONT,
  fillColor: FILL_COLOR_LIGHT,
});
const favicon_left = new IsometricRectangle({
  height: SHORT_SQUARE_PIECE_HEIGHT,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.SIDE,
  fillColor: FILL_COLOR_LIGHT,
});

// Position the piece centered
favicon_top.top = SHORT_PIECE_LENGTH_END;
favicon_top.left = centerOffset;
favicon_top.right = centerOffset;

favicon_right.top = PIECE_START;
favicon_right.right = SQUARE_PIECE_WIDTH + centerOffset;
favicon_right.left = centerOffset;

favicon_left.top = PIECE_START;
favicon_left.left = SQUARE_PIECE_WIDTH + centerOffset;
favicon_left.right = centerOffset;

// Cosmetic indent
const favicon_cosmetic_ident = new IsometricPath({
  fillColor: FILL_COLOR_LIGHT,
  autoclose: false,
});
favicon_cosmetic_ident
  .moveTo(
    SQUARE_PIECE_WIDTH + centerOffset,
    SQUARE_PIECE_WIDTH + centerOffset,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2,
  )
  .lineTo(
    SQUARE_PIECE_WIDTH + centerOffset,
    0 + centerOffset,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2,
  )
  .moveTo(
    SQUARE_PIECE_WIDTH + centerOffset,
    SQUARE_PIECE_WIDTH + centerOffset,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2,
  )
  .lineTo(
    0 + centerOffset,
    SQUARE_PIECE_WIDTH + centerOffset,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2,
  )
  .moveTo(
    SQUARE_PIECE_WIDTH + centerOffset,
    SQUARE_PIECE_WIDTH + centerOffset,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2,
  )
  .lineTo(
    SQUARE_PIECE_WIDTH + centerOffset,
    0 + centerOffset,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2,
  )
  .moveTo(
    SQUARE_PIECE_WIDTH + centerOffset,
    SQUARE_PIECE_WIDTH + centerOffset,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2,
  )
  .lineTo(
    0 + centerOffset,
    SQUARE_PIECE_WIDTH + centerOffset,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2,
  );

// Hollow circle on top
const favicon_hollow = new IsometricCircle({
  radius: CIRCLE_HOLE_RADIUS,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_HOLLOW,
});
// For square pieces, the hollow should be at the same height as the top
// plus we need to account for centering
favicon_hollow.top = SHORT_PIECE_LENGTH_END;
favicon_hollow.left = SQUARE_PIECE_WIDTH / 2 + centerOffset;
favicon_hollow.right = SQUARE_PIECE_WIDTH / 2 + centerOffset;

favicon_canvas
  .addChild(favicon_top)
  .addChild(favicon_right)
  .addChild(favicon_left)
  .addChild(favicon_cosmetic_ident)
  .addChild(favicon_hollow);

// Main async function
(async () => {
  const svgCode = saveSVG("favicon.svg", favicon_canvas);
  await convertToPNG(svgCode, "favicon.png", 512);
  console.log("\nFavicon generated successfully!");
})();
