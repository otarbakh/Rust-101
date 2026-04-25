import * as nodes from '@codama/nodes';
import { rootNodeFromAnchor } from '@codama/nodes-anchor';
import * as renderersJs from '@codama/renderers-js';
import * as visitorsCore from '@codama/visitors-core';
import * as fs from 'fs';

async function generate() {
    const idlPath = "./target/idl/movie_review_program.json";
    const outputPath = "./client/generated";

    if (!fs.existsSync(idlPath)) {
        console.error("❌ IDL file not found!");
        return;
    }

    const idl = JSON.parse(fs.readFileSync(idlPath, 'utf8'));

    // rootNodeFromAnchor არის Codama-ს სპეციალური ფუნქცია Anchor IDL-ისთვის.
    // ის ავტომატურად ამატებს ყველა საჭირო ველს (მათ შორის additionalPrograms-ს).
    const node = rootNodeFromAnchor(idl);

    visitorsCore.visit(node, renderersJs.renderVisitor(outputPath));

    console.log('✅ Successfully generated files in: ' + outputPath);
}

generate().catch(err => {
    console.error('❌ Error during generation:', err);
});