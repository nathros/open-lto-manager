export default {
	plugins: [
		{
			name: "preset-default",
		},
		{
			name: "removeDimensions", // Remove width and height from top level svg
		},
		{
			name: "removeXMLNS",
		},
		{
			name: "removeXlink",
		},
		{
			name: "cleanupIds",
			params: {
				minify: false, // Leave ids alone will be updated via prefixIds bellow
				remove: true, // Remove unreferences ids
				force: true, // Do not abort for ids in <script> and <style>
			},
		},
		{
			name: "prefixIds",
			params: {
				delim: "-",
				prefix: () => process.env.SVGO_PREFIX.replaceAll(" ", "_"), // id cannot contain spaces
			},
		},
	],
};
