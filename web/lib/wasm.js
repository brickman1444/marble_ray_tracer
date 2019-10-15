let rayLoaded = false;
let _render;

import('ray').then((Ray) => {

    rayLoaded = true;

    _render = function (event_data) {
        return Ray.binding(
            JSON.stringify(event_data.scene),
            event_data.width,
            event_data.height,
            event_data.x_pixel_start,
            event_data.x_pixel_end,
            event_data.y_pixel_start,
            event_data.y_pixel_end);
    };
});

export function render(event_data) {
    if (rayLoaded) {
        return _render(event_data);
    }
}