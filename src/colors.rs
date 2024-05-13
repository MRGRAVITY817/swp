// Default color names from Tailwind CSS color palette
const COLOR_NAMES: [&str; 27] = [
    "current",
    "inherit",
    "transparent",
    "black",
    "white",
    "slate",
    "gray",
    "zinc",
    "neutral",
    "stone",
    "red",
    "orange",
    "amber",
    "yellow",
    "lime",
    "green",
    "emerald",
    "teal",
    "cyan",
    "sky",
    "blue",
    "indigo",
    "violet",
    "purple",
    "fuchsia",
    "pink",
    "rose",
];

const COLOR_SCALES: [i16; 11] = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950];

const COLOR_OPACITIES: [i16; 21] = [
    0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100,
];

const COLORABLE_ELEMENTS: [&str; 16] = [
    "text",
    "decoration",
    "ring",
    "ring-offset",
    "outline",
    "divide",
    "border",
    "from",
    "via",
    "to",
    "bg",
    "caret",
    "accent",
    "shadow",
    "fill",
    "stroke",
];
