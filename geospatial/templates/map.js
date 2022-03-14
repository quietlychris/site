var map = new maplibregl.Map({
  container: "map",
  style:
    "https://api.maptiler.com/maps/hybrid/style.json?key=get_your_own_OpIi9ZULNHzrESv6T2vL",
  maxZoom: 19,
  minZoom: 2,
  zoom: 2.5,
  center: [-98.579533, 39.828527], // Historical geographic center of the United States
});

// On map load, pull in the data
map.on("load", function () {
  map.loadImage(
    "https://maplibre.org/maplibre-gl-js-docs/assets/custom_marker.png",
    // Add an image to use as a custom marker
    function (error, image) {
      if (error) throw error;
      map.addImage("custom-marker", image);

      map.addSource("places", {
        type: "geojson",
        data: [data:feature_collection]
      });

      // Add a layer showing the places.
      map.addLayer({
        id: "places",
        type: "symbol",
        source: "places",
        layout: {
          "icon-image": "custom-marker",
          "icon-overlap": "always",
        },
      });      

      [data:all_images]

    }
  );

  // Create a popup, but don't add it to the map yet.
  var div = document.createElement("div");
  var popup = new maplibregl.Popup({
    closeButton: true,
    closeOnClick: false,
  });

  map.on("mouseenter", "places", function (e) {
    // Change the cursor style as a UI indicator.
    map.getCanvas().style.cursor = "pointer";

    var coordinates = e.features[0].geometry.coordinates.slice();
    var description = e.features[0].properties.description;

    // Ensure that if the map is zoomed out such that multiple
    // copies of the feature are visible, the popup appears
    // over the copy being pointed to.
    while (Math.abs(e.lngLat.lng - coordinates[0]) > 180) {
      coordinates[0] += e.lngLat.lng > coordinates[0] ? 360 : -360;
    }

    // Populate the popup and set its coordinates
    // based on the feature found.
    popup.setLngLat(coordinates).setHTML(description).addTo(map);
  });

  map.on("click", "places", function (e) {
    map.flyTo({
      center: e.features[0].geometry.coordinates,
      zoom: 16,
    });
  });
});
