CREATE TABLE teacher_comments (
	id SERIAL PRIMARY KEY,
	author_id INT REFERENCES students ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
	receiver_id INT REFERENCES teachers ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
	body TEXT NOT NULL,
	published TIMESTAMP DEFAULT NOW() NOT NULL
);

CREATE TABLE teacher_comment_votes (
	id SERIAL PRIMARY KEY,
	comment_id INT REFERENCES teacher_comments ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
	student_id INT REFERENCES students ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
	upvote BOOLEAN NOT NULL, -- true -> upvote; false -> downvote, no row -> no vote
	UNIQUE(comment_id, student_id)
);
