import * as web3 from '@solana/web3.js';
import { 
    getAddMovieReviewInstruction, 
    getUpdateMovieReviewInstruction, 
    getDeleteMovieReviewInstruction,
    fetchMovieReview,
    findMovieReviewPda
} from './client/generated';

async function main() {
    // 1. დაკავშირება ლოკალურ კლასტერთან (ან Devnet-თან)
    const connection = new web3.Connection("http://127.0.0.1:8899", "confirmed");
    
    // 2. მომხმარებლის მომზადება (გამოვიყენოთ ლოკალური anchor wallet)
    const payer = web3.Keypair.generate(); // ტესტისთვის ახალი ვოლეტი
    
    // აირდროპი ტესტისთვის (მხოლოდ ლოკალჰოსტზე ან დევნეტზე)
    const airdropSig = await connection.requestAirdrop(payer.publicKey, 2 * web3.LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSig);

    const movieTitle = "Inception";
    const [moviePda] = await findMovieReviewPda({ 
        title: movieTitle, 
        reviewer: payer.publicKey 
    });

    console.log(`🚀 Starting E2E Flow for: ${movieTitle}`);

    // --- ADD REVIEW ---
    const addIx = getAddMovieReviewInstruction({
        movieReview: moviePda,
        initializer: payer.publicKey,
        title: movieTitle,
        description: "Mind-bending masterpiece!",
        rating: 10,
    });

    const txAdd = new web3.Transaction().add(addIx);
    await web3.sendAndConfirmTransaction(connection, txAdd, [payer]);
    console.log("✅ Review added successfully!");

    // --- FETCH REVIEW ---
    let reviewData = await fetchMovieReview(connection, moviePda);
    console.log("📊 Fetched Review:", {
        title: reviewData.data.title,
        rating: reviewData.data.rating,
        description: reviewData.data.description
    });

    // --- UPDATE REVIEW ---
    const updateIx = getUpdateMovieReviewInstruction({
        movieReview: moviePda,
        initializer: payer.publicKey,
        reviewer: payer.publicKey,
        title: movieTitle,
        newDescription: "Still great after second watch!",
        newRating: 9,
    });

    const txUpdate = new web3.Transaction().add(updateIx);
    await web3.sendAndConfirmTransaction(connection, txUpdate, [payer]);
    console.log("🔄 Review updated successfully!");

    // --- FETCH UPDATED ---
    reviewData = await fetchMovieReview(connection, moviePda);
    console.log("📊 Fetched Updated Review:", {
        rating: reviewData.data.rating,
        description: reviewData.data.description
    });

    // --- DELETE REVIEW ---
    const deleteIx = getDeleteMovieReviewInstruction({
        movieReview: moviePda,
        initializer: payer.publicKey,
        reviewer: payer.publicKey,
        title: movieTitle,
    });

    const txDelete = new web3.Transaction().add(deleteIx);
    await web3.sendAndConfirmTransaction(connection, txDelete, [payer]);
    console.log("🗑️ Review deleted. Lamports returned to payer.");

    try {
        await fetchMovieReview(connection, moviePda);
    } catch (e) {
        console.log("📌 Confirmation: Account no longer exists.");
    }
}

main().catch(console.error);