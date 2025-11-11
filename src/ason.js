const { SmartCompressor, TokenCounter } = require('@ason-format/ason');

/**
 * ASON Extension for Zed
 *
 * Provides commands to compress/decompress JSON using ASON format
 */

// Global configuration (can be customized)
let globalConfig = {
  indent: 1,
  delimiter: ',',
  useReferences: true,
  useDictionary: true
};

/**
 * Compress JSON to ASON
 * @param {string} jsonText - JSON text to compress
 * @returns {object} - Result with ASON text and stats
 */
function compressJson(jsonText) {
  try {
    const compressor = new SmartCompressor(globalConfig);
    const data = JSON.parse(jsonText);
    const ason = compressor.compress(data);
    const stats = TokenCounter.compareFormats(data, ason);

    return {
      success: true,
      ason,
      stats: {
        originalTokens: stats.original_tokens,
        compressedTokens: stats.compressed_tokens,
        reductionPercent: stats.reduction_percent.toFixed(2),
        originalSize: stats.original_size,
        compressedSize: stats.compressed_size,
        savings: stats.original_size - stats.compressed_size
      }
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}

/**
 * Decompress ASON to JSON
 * @param {string} asonText - ASON text to decompress
 * @returns {object} - Result with JSON text
 */
function decompressAson(asonText) {
  try {
    const compressor = new SmartCompressor();
    const data = compressor.decompress(asonText);
    const json = JSON.stringify(data, null, 2);

    return {
      success: true,
      json
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}

/**
 * Get compression statistics
 * @param {string} jsonText - JSON text to analyze
 * @returns {object} - Statistics result
 */
function getStats(jsonText) {
  try {
    const compressor = new SmartCompressor(globalConfig);
    const data = JSON.parse(jsonText);
    const ason = compressor.compress(data);
    const stats = TokenCounter.compareFormats(data, ason);

    return {
      success: true,
      stats: {
        originalTokens: stats.original_tokens,
        compressedTokens: stats.compressed_tokens,
        reductionPercent: stats.reduction_percent.toFixed(2),
        originalSize: stats.original_size,
        compressedSize: stats.compressed_size,
        savings: stats.original_size - stats.compressed_size
      },
      asonPreview: ason,
      config: globalConfig
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}

/**
 * Configure ASON compressor
 * @param {object} config - New configuration
 */
function configure(config) {
  globalConfig = { ...globalConfig, ...config };
  return {
    success: true,
    config: globalConfig
  };
}

// Export functions for Zed extension API
module.exports = {
  compressJson,
  decompressAson,
  getStats,
  configure
};
