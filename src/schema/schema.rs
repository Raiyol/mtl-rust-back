// @generated automatically by Diesel CLI.

diesel::table! {
    bookmark (id_user, id_novel) {
        id_user -> Unsigned<Integer>,
        id_novel -> Unsigned<Integer>,
        chapter -> Unsigned<Integer>,
    }
}

diesel::table! {
    chapter (id) {
        id -> Unsigned<Integer>,
        id_novel -> Unsigned<Integer>,
        number -> Unsigned<Integer>,
        date -> Nullable<Datetime>,
        title_en -> Nullable<Varchar>,
        title_cn -> Nullable<Varchar>,
        content -> Nullable<Longtext>,
        dict -> Nullable<Longtext>,
    }
}

diesel::table! {
    comment (id) {
        id -> Unsigned<Integer>,
        id_chapter -> Unsigned<Integer>,
        id_user -> Unsigned<Integer>,
        text -> Varchar,
        date -> Nullable<Datetime>,
    }
}

diesel::table! {
    genre (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
    }
}

diesel::table! {
    novel (id) {
        id -> Unsigned<Integer>,
        url -> Varchar,
        name -> Varchar,
        cn_name -> Varchar,
        author -> Nullable<Varchar>,
        summary -> Nullable<Text>,
        img -> Nullable<Varchar>,
        date -> Nullable<Datetime>,
        completed -> Tinyint,
    }
}

diesel::table! {
    novel_genre (id_novel, id_genre) {
        id_novel -> Unsigned<Integer>,
        id_genre -> Unsigned<Integer>,
    }
}

diesel::table! {
    review (id) {
        id -> Unsigned<Integer>,
        id_novel -> Unsigned<Integer>,
        id_user -> Unsigned<Integer>,
        rate -> Nullable<Integer>,
        text -> Nullable<Text>,
        date -> Nullable<Datetime>,
    }
}

diesel::table! {
    review_user_liked (id_review, id_user) {
        id_review -> Unsigned<Integer>,
        id_user -> Unsigned<Integer>,
    }
}

diesel::table! {
    user (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        email -> Varchar,
        pwd -> Varchar,
        confirmed -> Tinyint,
        role -> Nullable<Varchar>,
        pfp -> Varchar,
        date -> Nullable<Datetime>,
    }
}

diesel::joinable!(bookmark -> novel (id_novel));
diesel::joinable!(bookmark -> user (id_user));
diesel::joinable!(chapter -> novel (id_novel));
diesel::joinable!(novel_genre -> genre (id_genre));
diesel::joinable!(novel_genre -> novel (id_novel));
diesel::joinable!(review -> novel (id_novel));
diesel::joinable!(review -> user (id_user));
diesel::joinable!(review_user_liked -> review (id_review));
diesel::joinable!(review_user_liked -> user (id_user));

diesel::allow_tables_to_appear_in_same_query!(
    bookmark,
    chapter,
    comment,
    genre,
    novel,
    novel_genre,
    review,
    review_user_liked,
    user,
);
