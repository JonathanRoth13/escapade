const CANVAS_BACKGROUND_COLOR = "none";
const CANVAS_SCALE = 100;
const CANVAS_WIDTH = 1001.5;
const CANVAS_HEIGHT = 1001.5;

const FILL_COLOR_LIGHT = "#D5B99E";
const FILL_COLOR_DARK = "#686868";
const FILL_COLOR_HOLLOW = "#000";
const SQUARE_PIECE_WIDTH = 3;
const TALL_SQUARE_PIECE_HEIGHT = 7;
const SHORT_SQUARE_PIECE_HEIGHT = 4;
const CIRCLE_PIECE_RADIUS = SQUARE_PIECE_WIDTH / 2;
const CIRCLE_PIECE_OFFSET = -(SQUARE_PIECE_WIDTH / 2);
const CIRCLE_PIECE_EDGE = Math.sqrt(Math.pow(CIRCLE_PIECE_RADIUS, 2) / 2);

const COSMETIC_INDENT_HEIGHT = 1;
const COSMETIC_INDENT_LENGTH = 0.4;

const PIECE_START = -2;
const TALL_PIECE_LENGTH_END = 5;
const SHORT_PIECE_LENGTH_END = 2;

const CIRCLE_HOLE_RADIUS = 1;

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
/* eslint-enable @typescript-eslint/no-require-imports */

// Create output directory
const outputDir = path.join(__dirname, "../public/pieces");
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
}

// Piece 0: Tall, Light, Solid, Circle
const piece_00_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
const piece_00_top = new IsometricCircle({
  radius: CIRCLE_PIECE_RADIUS,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_LIGHT,
});
const piece_00_outline = new IsometricPath({ fillColor: FILL_COLOR_LIGHT });
const piece_00_cosmetic_ident = new IsometricPath({
  fillColor: FILL_COLOR_LIGHT,
  autoclose: false,
});
piece_00_top.top = TALL_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET;
piece_00_outline
  .moveTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    TALL_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET,
  )
  .lineTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
  )
  .curveTo(
    CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
  )
  .lineTo(
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    TALL_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET,
  );
piece_00_cosmetic_ident
  .moveTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  )
  .curveTo(
    CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  )
  .moveTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  )
  .curveTo(
    CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  );
piece_00_canvas
  .addChild(piece_00_outline)
  .addChild(piece_00_cosmetic_ident)
  .addChild(piece_00_top);
saveSVG("piece0.svg", piece_00_canvas);

// Piece 1: Tall, Light, Hollow, Circle
const piece_01_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
const piece_01_hollow = new IsometricCircle({
  radius: CIRCLE_HOLE_RADIUS,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_HOLLOW,
});
piece_01_hollow.top = TALL_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET;
piece_01_canvas
  .addChild(piece_00_outline)
  .addChild(piece_00_cosmetic_ident)
  .addChild(piece_00_top)
  .addChild(piece_01_hollow);
saveSVG("piece1.svg", piece_01_canvas);

// Piece 2: Short, Light, Solid, Circle
const piece_02_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
const piece_02_top = new IsometricCircle({
  radius: CIRCLE_PIECE_RADIUS,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_LIGHT,
});
const piece_02_outline = new IsometricPath({ fillColor: FILL_COLOR_LIGHT });

piece_02_top.top = SHORT_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET;
piece_02_outline
  .moveTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    SHORT_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET,
  )
  .lineTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
  )
  .curveTo(
    CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
  )
  .lineTo(
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    SHORT_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET,
  );

piece_02_canvas
  .addChild(piece_02_outline)
  .addChild(piece_00_cosmetic_ident)
  .addChild(piece_02_top);
saveSVG("piece2.svg", piece_02_canvas);

// Piece 3: Short, Light, Hollow, Circle
const piece_03_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
const piece_03_hollow = new IsometricCircle({
  radius: CIRCLE_HOLE_RADIUS,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_HOLLOW,
});

piece_03_hollow.top = SHORT_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET;
piece_03_canvas
  .addChild(piece_02_outline)
  .addChild(piece_00_cosmetic_ident)
  .addChild(piece_02_top)
  .addChild(piece_03_hollow);
saveSVG("piece3.svg", piece_03_canvas);

// Piece 4: Tall, Light, Solid, Square
const piece_04_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
const piece_04_top = new IsometricRectangle({
  height: SQUARE_PIECE_WIDTH,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_LIGHT,
});
const piece_04_right = new IsometricRectangle({
  height: TALL_SQUARE_PIECE_HEIGHT,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.FRONT,
  fillColor: FILL_COLOR_LIGHT,
});
const piece_04_left = new IsometricRectangle({
  height: TALL_SQUARE_PIECE_HEIGHT,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.SIDE,
  fillColor: FILL_COLOR_LIGHT,
});
piece_04_top.top = TALL_PIECE_LENGTH_END;
piece_04_right.top = PIECE_START;
piece_04_right.right = SQUARE_PIECE_WIDTH;
piece_04_left.top = PIECE_START;
piece_04_left.left = SQUARE_PIECE_WIDTH;
const piece_04_cosmetic_ident = new IsometricPath({
  fillColor: FILL_COLOR_LIGHT,
  autoclose: false,
});
piece_04_cosmetic_ident
  .moveTo(
    SQUARE_PIECE_WIDTH,
    SQUARE_PIECE_WIDTH,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2,
  )
  .lineTo(
    SQUARE_PIECE_WIDTH,
    0,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2,
  )
  .moveTo(
    SQUARE_PIECE_WIDTH,
    SQUARE_PIECE_WIDTH,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2,
  )
  .lineTo(
    0,
    SQUARE_PIECE_WIDTH,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2,
  )
  .moveTo(
    SQUARE_PIECE_WIDTH,
    SQUARE_PIECE_WIDTH,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2,
  )
  .lineTo(
    SQUARE_PIECE_WIDTH,
    0,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2,
  )
  .moveTo(
    SQUARE_PIECE_WIDTH,
    SQUARE_PIECE_WIDTH,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2,
  )
  .lineTo(
    0,
    SQUARE_PIECE_WIDTH,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2,
  );
piece_04_canvas
  .addChild(piece_04_top)
  .addChild(piece_04_right)
  .addChild(piece_04_left)
  .addChild(piece_04_cosmetic_ident);
saveSVG("piece4.svg", piece_04_canvas);

// Piece 5: Tall, Light, Hollow, Square
const piece_05_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
piece_05_canvas
  .addChild(piece_04_top)
  .addChild(piece_04_right)
  .addChild(piece_04_left)
  .addChild(piece_04_cosmetic_ident)
  .addChild(piece_01_hollow);
saveSVG("piece5.svg", piece_05_canvas);

// Piece 6: Short, Light, Solid, Square
const piece_06_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
const piece_06_top = new IsometricRectangle({
  height: SQUARE_PIECE_WIDTH,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_LIGHT,
});
const piece_06_right = new IsometricRectangle({
  height: SHORT_SQUARE_PIECE_HEIGHT,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.FRONT,
  fillColor: FILL_COLOR_LIGHT,
});
const piece_06_left = new IsometricRectangle({
  height: SHORT_SQUARE_PIECE_HEIGHT,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.SIDE,
  fillColor: FILL_COLOR_LIGHT,
});
piece_06_top.top = SHORT_PIECE_LENGTH_END;
piece_06_right.top = PIECE_START;
piece_06_right.right = SQUARE_PIECE_WIDTH;
piece_06_left.top = PIECE_START;
piece_06_left.left = SQUARE_PIECE_WIDTH;
piece_06_canvas
  .addChild(piece_06_top)
  .addChild(piece_06_right)
  .addChild(piece_06_left)
  .addChild(piece_04_cosmetic_ident);
saveSVG("piece6.svg", piece_06_canvas);

// Piece 7: Short, Light, Hollow, Square
const piece_07_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
piece_07_canvas
  .addChild(piece_06_top)
  .addChild(piece_06_right)
  .addChild(piece_06_left)
  .addChild(piece_04_cosmetic_ident)
  .addChild(piece_03_hollow);
saveSVG("piece7.svg", piece_07_canvas);

// Piece 8: Tall, Dark, Solid, Circle
const piece_08_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
const piece_08_top = new IsometricCircle({
  radius: CIRCLE_PIECE_RADIUS,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_DARK,
});
const piece_08_outline = new IsometricPath({ fillColor: FILL_COLOR_DARK });
const piece_08_cosmetic_ident = new IsometricPath({
  fillColor: FILL_COLOR_DARK,
  autoclose: false,
});
piece_08_top.top = TALL_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET;
piece_08_outline
  .moveTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    TALL_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET,
  )
  .lineTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
  )
  .curveTo(
    CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
  )
  .lineTo(
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    TALL_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET,
  );
piece_08_cosmetic_ident
  .moveTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  )
  .curveTo(
    CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  )
  .moveTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  )
  .curveTo(
    CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  );
piece_08_canvas
  .addChild(piece_08_outline)
  .addChild(piece_08_cosmetic_ident)
  .addChild(piece_08_top);
saveSVG("piece8.svg", piece_08_canvas);

// Piece 9: Tall, Dark, Hollow, Circle
const piece_09_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
piece_09_canvas
  .addChild(piece_08_outline)
  .addChild(piece_08_cosmetic_ident)
  .addChild(piece_08_top)
  .addChild(piece_01_hollow);
saveSVG("piece9.svg", piece_09_canvas);

// Piece 10: Short, Dark, Solid, Circle
const piece_10_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
const piece_10_top = new IsometricCircle({
  radius: CIRCLE_PIECE_RADIUS,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_DARK,
});
const piece_10_outline = new IsometricPath({ fillColor: FILL_COLOR_DARK });
const piece_10_cosmetic_ident = new IsometricPath({
  fillColor: FILL_COLOR_DARK,
  autoclose: false,
});
piece_10_cosmetic_ident
  .moveTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  )
  .curveTo(
    CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT + COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  )
  .moveTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  )
  .curveTo(
    CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    COSMETIC_INDENT_HEIGHT - COSMETIC_INDENT_LENGTH / 2 + CIRCLE_PIECE_OFFSET,
  );
piece_10_top.top = SHORT_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET;
piece_10_outline
  .moveTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    SHORT_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET,
  )
  .lineTo(
    CIRCLE_PIECE_EDGE,
    -CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
  )
  .curveTo(
    CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    PIECE_START + CIRCLE_PIECE_OFFSET,
  )
  .lineTo(
    -CIRCLE_PIECE_EDGE,
    CIRCLE_PIECE_EDGE,
    SHORT_PIECE_LENGTH_END + CIRCLE_PIECE_OFFSET,
  );

piece_10_canvas
  .addChild(piece_10_outline)
  .addChild(piece_10_cosmetic_ident)
  .addChild(piece_10_top);
saveSVG("piece10.svg", piece_10_canvas);

// Piece 11: Short, Dark, Hollow, Circle
const piece_11_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});

piece_11_canvas
  .addChild(piece_10_outline)
  .addChild(piece_10_cosmetic_ident)
  .addChild(piece_10_top)
  .addChild(piece_03_hollow);
saveSVG("piece11.svg", piece_11_canvas);

// Piece 12: Tall, Dark, Solid, Square
const piece_12_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
const piece_12_top = new IsometricRectangle({
  height: SQUARE_PIECE_WIDTH,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_DARK,
});
const piece_12_right = new IsometricRectangle({
  height: TALL_SQUARE_PIECE_HEIGHT,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.FRONT,
  fillColor: FILL_COLOR_DARK,
});
const piece_12_left = new IsometricRectangle({
  height: TALL_SQUARE_PIECE_HEIGHT,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.SIDE,
  fillColor: FILL_COLOR_DARK,
});
piece_12_top.top = TALL_PIECE_LENGTH_END;
piece_12_right.top = PIECE_START;
piece_12_right.right = SQUARE_PIECE_WIDTH;
piece_12_left.top = PIECE_START;
piece_12_left.left = SQUARE_PIECE_WIDTH;

piece_12_canvas
  .addChild(piece_12_top)
  .addChild(piece_12_right)
  .addChild(piece_12_left)
  .addChild(piece_04_cosmetic_ident);
saveSVG("piece12.svg", piece_12_canvas);

// Piece 13: Tall, Dark, Hollow, Square
const piece_13_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
piece_13_canvas
  .addChild(piece_12_top)
  .addChild(piece_12_right)
  .addChild(piece_12_left)
  .addChild(piece_04_cosmetic_ident)
  .addChild(piece_01_hollow);
saveSVG("piece13.svg", piece_13_canvas);

// Piece 14: Short, Dark, Solid, Square
const piece_14_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
const piece_14_top = new IsometricRectangle({
  height: SQUARE_PIECE_WIDTH,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.TOP,
  fillColor: FILL_COLOR_DARK,
});
const piece_14_right = new IsometricRectangle({
  height: SHORT_SQUARE_PIECE_HEIGHT,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.FRONT,
  fillColor: FILL_COLOR_DARK,
});
const piece_14_left = new IsometricRectangle({
  height: SHORT_SQUARE_PIECE_HEIGHT,
  width: SQUARE_PIECE_WIDTH,
  planeView: PlaneView.SIDE,
  fillColor: FILL_COLOR_DARK,
});
piece_14_top.top = SHORT_PIECE_LENGTH_END;
piece_14_right.top = PIECE_START;
piece_14_right.right = SQUARE_PIECE_WIDTH;
piece_14_left.top = PIECE_START;
piece_14_left.left = SQUARE_PIECE_WIDTH;
piece_14_canvas
  .addChild(piece_14_top)
  .addChild(piece_14_right)
  .addChild(piece_14_left)
  .addChild(piece_04_cosmetic_ident);
saveSVG("piece14.svg", piece_14_canvas);

// Piece 15: Short, Dark, Hollow, Square
const piece_15_canvas = new IsometricCanvas({
  backgroundColor: CANVAS_BACKGROUND_COLOR,
  scale: CANVAS_SCALE,
  width: CANVAS_WIDTH,
  height: CANVAS_HEIGHT,
});
piece_15_canvas
  .addChild(piece_14_top)
  .addChild(piece_14_right)
  .addChild(piece_14_left)
  .addChild(piece_04_cosmetic_ident)
  .addChild(piece_03_hollow);
saveSVG("piece15.svg", piece_15_canvas);

console.log("\nAll pieces generated successfully!");
