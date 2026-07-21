const { execSync } = require('child_process');

const shouldCompile = process.env.FAST_PIXELATE_COMPILE === 'true';

if (shouldCompile) {
	console.log('[fast-pixelate] Build flag detected. Compilation from source begins...');
	try {
		execSync('npm run build', { stdio: 'inherit' });
		console.log('[fast-pixelate] Assembly completed successfully!');
	} catch (error) {
		console.error('[fast-pixelate] Error when building from source:', error.message);
		process.exit(1);
	}
} else {
	console.log('[fast-pixelate] Precompiled binaries are used.');
}
