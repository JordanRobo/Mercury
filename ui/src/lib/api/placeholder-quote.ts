export function random_quote(): string {
	let i = Math.floor(Math.random() * quotes.length);
	return quotes[i];
}

let quotes = [
	"If there’s a book that you want to read, but it hasn’t been written yet, then you must write it. - Toni Morrison",
	"Either write something worth reading or do something worth writing. - Benjamin Franklin",
	"If a story is in you, it has to come out. - William Faulkner",
	"I love writing. I love the swirl and swing of words as they tangle with human emotions. - James Michener",
	"Your intuition knows what to write, so get out of the way. - Ray Bradbury",
	"A writer, I think, is someone who pays attention to the world. - Susan Sontag",
	"The scariest moment is always just before you start. - Stephen King",
	"Fill your paper with the breathings of your heart. - William Wordsworth",
	"That's the thing about books, they let you travel without moving your feet. - Jhumpa Lahiri",
	"And now that you don't have to be perfect, you can be good. - John Steinbeck",
	"I don't wait for moods. You accomplish nothing if you do that. Your mind must know it has got to get down to work. - Pearl S. Buck",
	"No need to force yourself to do something the 'right way' if it's not your right way. Your job is to honor your process. - Andi Cumbo",
	"Writing is a calling, not a choice. - Isabel Allende",
	"For your born writer, nothing is so healing as the realization that he has come upon the right word. - Catherine Drinker Bowen",
	"Instructions for living a life. Pay attention. Be astonished. Tell about it. - Mary Oliver",
	"When I write, I give people access to their own emotions. - Gord Downie",
	"Lying in front of the house all afternoon, trying to write a poem. Falling asleep. Waking up under the stars. - Jack Gilbert",
	"Give me books, fruit, French wine, fine weather and a little music. - John Keats",
	"The idea is to write it so that people hear it and it slides through the brain and goes straight to the heart. - Maya Angelou",
	"We write to taste life twice, in the moment and in retrospect. - Anaïs Nin",
	"Start writing, no matter what. The water does not flow until the faucet is turned on. - Louis L'Amour",
	"There is something delicious about writing the first words of a story. You never quite know where they'll take you. - Beatrix Potter",
	"A word after a word after a word is power. - Margaret Atwood",
	"The purpose of a writer is to keep civilization from destroying itself. - Albert Camus",
	"My aim is to put down on paper what I see and what I feel in the best and simplest way. - Ernest Hemingway",
];
