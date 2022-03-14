      // Add the images
      map.addSource("[data:dataset_name]", {
        type: "image",
        // url: getPath(),
        url: "/geospatial/data/[data:dataset_name]/odm_orthophoto.webp",
        coordinates: [
          [[data:min_x], [data:max_y]],
          [[data:max_x], [data:max_y]],
          [[data:max_x], [data:min_y]],
          [[data:min_x], [data:min_y]],
        ],
      });
      map.addLayer({
        id: "[data:dataset_name]",
        type: "raster",
        source: "[data:dataset_name]",
      });
