// Based on jito restaking client generation.

const codama = require("codama");
const anchorIdl = require("@codama/nodes-from-anchor");
const path = require("path");
const renderers = require('@codama/renderers');

// Paths.
const projectRoot = path.join(__dirname, "..");

const idlDir = path.join(projectRoot, "idl");

const jsClientsDir = path.join(__dirname, "..", "clients", "js");
const umiClientsDir = path.join(__dirname, "..", "clients", "umi");

// Generate the restaking client in Rust and JavaScript.
const jsClientDir = path.join(jsClientsDir, "vault_client");
const umiClientDir = path.join(umiClientsDir, "vault_client");
const vaultRootNode = anchorIdl.rootNodeFromAnchor(require(path.join(idlDir, "vault.json")));
const vaultCodama = codama.createFromRoot(vaultRootNode);
// vaultCodama.update(codama.bottomUpTransformerVisitor([
//     {
//         // PodU64 -> u64
//         select: (node) => {
//             return (
//                 codama.isNode(node, "structFieldTypeNode") &&
//                 node.type.name === "podU64"
//             );
//         },
//         transform: (node) => {
//             codama.assertIsNode(node, "structFieldTypeNode");
//             return {
//                 ...node,
//                 type: codama.numberTypeNode("u64"),
//             };
//         },
//     },
//     {
//         // PodU32 -> u32
//         select: (node) => {
//             return (
//                 codama.isNode(node, "structFieldTypeNode") &&
//                 node.type.name === "podU32"
//             );
//         },
//         transform: (node) => {
//             codama.assertIsNode(node, "structFieldTypeNode");
//             return {
//                 ...node,
//                 type: codama.numberTypeNode("u32"),
//             };
//         },
//     },
//     {
//         // PodU16 -> u16
//         select: (node) => {
//             return (
//                 codama.isNode(node, "structFieldTypeNode") &&
//                 node.type.name === "podU16"
//             );
//         },
//         transform: (node) => {
//             codama.assertIsNode(node, "structFieldTypeNode");
//             return {
//                 ...node,
//                 type: codama.numberTypeNode("u16"),
//             };
//         },
//     },
//     // add 8 byte discriminator to accountNode
//     {
//         select: (node) => {
//             return (
//                 codama.isNode(node, "accountNode")
//             );
//         },
//         transform: (node) => {
//             codama.assertIsNode(node, "accountNode");

//             return {
//                 ...node,
//                 data: {
//                     ...node.data,
//                     fields: [
//                         codama.structFieldTypeNode({name: 'discriminator', type: codama.numberTypeNode('u64')}),
//                         ...node.data.fields
//                     ]
//                 }
//             };
//         },
//     },
// ]));
vaultCodama.accept(renderers.renderJavaScriptUmiVisitor(path.join(umiClientDir, "src", "generated"), {
    formatCode: true,
    crateFolder: umiClientDir,
    deleteFolderBeforeRendering: true,
}));
vaultCodama.accept(renderers.renderJavaScriptVisitor(path.join(jsClientDir), {}));
