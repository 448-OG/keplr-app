## keplr
## celestia

### Create the Rust library
Create the Rust library in the root directory for easier builds. You can see the changes made to this project that enable running Rust web assembly modules by diffing the commit
`efb0e2d0e77f9ed96a2c0354a0effa13e9169112`.

### Building:
1. Ensure you have installed wasm-pack and then build the Rust library to a webassembly module
    ```sh
    $ npm run build:wasm
    ```
2. Ensure that the package is added as a dependency in the `package.json` file
    ```json
      "scripts": {
        //...previous parts of the script here
    +    "build:wasm": "cd wasm-lib && wasm-pack build --target web --out-dir pkg"
    },
    ```
3. Install our dependency
    ```sh
    $ npm install
    ```
4. In our `App.tsx` file.
    ```ts
    // Import the package, remember to also import
    // `init` as it is used to initialize our wasm module 
    import init, { hello, adder } from "wasm-lib";

    // Already existing typescript code here

    const App: React.FC = () => {
    useEffect(() => {
        async function main() {
        // Already existing typescript code here

        init().then(() => {
            console.log(hello());
            console.log(adder(1, 2));
        })
        }

        main();
    }, []);

    return (
        <div className="App">
        <header className="App-header">
            <p>Hello!</p>
        </header>
        </div>
    );
    };

    export default App;
    ```

5. Run `npm start`

#### LICENSE
The commit `efb0e2d0e77f9ed96a2c0354a0effa13e9169112` is release to the public domain using `CC0-1.0` LICENSE.