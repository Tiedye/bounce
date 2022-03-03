module.exports = (config) => ({
  ...config,
  experiments: {
    asyncWebAssembly: true,
  },
});
