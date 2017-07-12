const wasm = require('./main.rs');

wasm.initialize().then(module => {

    // Create a javascript wrapper around our rust function
    // module.cwrap(methodName, returnType, [parameterTypes]);
    const fix_story = module.cwrap('fix_story', 'string', ['string']);
    const add       = module.cwrap('add', 'number', ['number', 'number']);
    const square    = module.cwrap('square', 'number', ['number']);

    console.log('String replacement:', fix_story("one upon a time..."));
    console.log('Add:', add(25, 35));
    console.log('Square:', square(25));

});