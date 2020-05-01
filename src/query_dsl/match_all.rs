//! This mod is to
//! https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-all-query.html
//!
//!The most simple query, which matches all documents, giving them all a _score of 1.0

mod dsl {
    use crate::zdbquery::ZDBQuery;
    use pgx::*;
    use serde_json::*;

    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn match_all(boost: default!(f32, 1.0)) -> ZDBQuery {
        if boost == 1.0 {
            ZDBQuery::new_with_query_dsl(json! {
                {
                    "match_all": { }
                }
            })
        } else {
            ZDBQuery::new_with_query_dsl(json! {
                {
                    "match_all": { "boost" : boost }
                }
            })
        }
    }

    #[pg_extern(immutable, parallel_safe)]
    pub(super) fn match_none() -> ZDBQuery {
        ZDBQuery::new_with_query_dsl(json! {
             {
               "match_none": { }
             }
        })
    }
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use crate::query_dsl::match_all::dsl::*;
    use crate::zdbquery::ZDBQuery;
    use pgx::*;
    use serde_json::json;

    #[pg_test]
    fn test_matchall_with_boost() {
        let zdbquery = match_all(42.0);
        let dls = zdbquery.query_dsl();

        assert!(dls.is_some());
        assert_eq!(
            dls.unwrap(),
            &json! {
                {
                    "match_all": { "boost": 42.0}
                }
            }
        );
    }

    #[pg_test]
    fn test_match_all_with_default() {
        let zdbquery = Spi::get_one::<ZDBQuery>("SELECT dsl.match_all();")
            .expect("didn't get SPI return value");
        let dls = zdbquery.query_dsl();

        assert!(dls.is_some());
        assert_eq!(
            dls.unwrap(),
            &json! {
                {
                    "match_all": { "boost": 1.0}
                }
            }
        );
    }

    #[pg_test]
    fn test_match_none() {
        let zdbquery = match_none();
        let dls = zdbquery.query_dsl();

        assert!(dls.is_some());
        assert_eq!(
            dls.unwrap(),
            &json! {
                {
                    "match_none": { }
                }
            }
        );
    }
}
