import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import type { SistemaInventario } from "../target/types/sistema_inventario";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.SistemaInventario as anchor.Program<SistemaInventario>;


async function main() {
  const program = program;
  const PRODUCTO_DEFAULT = "HackathonProduct";
  const PRECIO_DEFAULT = new anchor.BN(500);
  const STOCK_INICIAL = 100;

  const [globalPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("global")],
    program.programId
  );

  const [productoPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("prod"), Buffer.from(PRODUCTO_DEFAULT)],
    program.programId
  );

  try {
    await program.methods.inicializarInventario().accounts({
      estadoGlobal: globalPda,
      admin: program.provider.publicKey,
      systemProgram: web3.SystemProgram.programId,
    }).rpc();
    console.log("Sistema Inicializado");
  } catch (e) {}

  try {
    await program.methods.crearProducto(PRODUCTO_DEFAULT, PRECIO_DEFAULT, STOCK_INICIAL).accounts({
      producto: productoPda,
      usuario: program.provider.publicKey,
      systemProgram: web3.SystemProgram.programId,
    }).rpc();
    console.log("Producto creado");
  } catch (e) {}

  try {
    await program.methods.venderProducto(5).accounts({
      producto: productoPda,
      estadoGlobal: globalPda,
      usuario: program.provider.publicKey,
    }).rpc();
    console.log("Venta exitosa");
  } catch (e) {
    console.error("Error:", e.message);
  }

  const estadoG = await program.account.estadoGlobal.fetch(globalPda);
  const infoP = await program.account.producto.fetch(productoPda);

  console.log("Ventas Totales:", estadoG.totalVentas.toString());
  console.log("Producto:", infoP.nombre);
  console.log("Stock:", infoP.stock);
}

main();