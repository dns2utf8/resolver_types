//! This crate allows the abstratction of DNS resolvers such as
//! `trust-dns-resolver` or `GaiResolver` from hyper
//!

use async_trait::async_trait;
use std::iter::Iterator;
use std::net::IpAddr;

pub type ResolveResult<T> = Result<T, ResolveError>;

/// The simplified interface that all resolvers share
#[async_trait]
pub trait AsyncResolver {
    /// Resolve IPv6 and IPv4
    async fn resolve(&self) -> ResolveResult<IpAddr> {
        let queries = [QueryType::AAAA, QueryType::A];
        let records = self.resolve_many(queries.into_iter()).await?;
        // try get first element
        match records
            .into_iter()
            .filter_map(|record| match record {
                Record::IpAddr(ip) => Some(ip),
                _ => None,
            })
            .next()
        {
            Some(ip) => Ok(ip),
            None => Err(ResolveError::NotResolved),
        }
    }

    async fn resolve_specific(&self, query: QueryType) -> ResolveResult<Record>;
    async fn resolve_many<I: Iterator<Item = QueryType>>(
        &self,
        queries: I,
    ) -> ResolveResult<Vec<Record>>;

    /// Potentially clear the cache of the actual implementation
    ///
    /// Returns Ok if implemented
    async fn clear_cache(&self) -> Result<(), ()> {
        Err(())
    }
    /// If the system settings are cached reload them
    ///
    /// Returns ok if implemented
    async fn reload_system_config(&self) -> Result<(), ()> {
        Err(())
    }
}

/// The simplified interface that all resolvers share
pub trait Resolver {
    /// Resolve IPv6 and IPv4
    fn resolve(&self) -> ResolveResult<IpAddr> {
        let queries = [QueryType::AAAA, QueryType::A];
        let records = self.resolve_many(queries.into_iter())?;
        // try get first element
        match records
            .into_iter()
            .filter_map(|record| match record {
                Record::IpAddr(ip) => Some(ip),
                _ => None,
            })
            .next()
        {
            Some(ip) => Ok(ip),
            None => Err(ResolveError::NotResolved),
        }
    }

    fn resolve_specific(&self, query: QueryType) -> ResolveResult<Record>;
    fn resolve_many<I: Iterator<Item = QueryType>>(&self, queries: I)
        -> ResolveResult<Vec<Record>>;

    /// Potentially clear the cache of the actual implementation
    ///
    /// Returns Ok if implemented
    fn clear_cache(&self) -> Result<(), ()> {
        Err(())
    }
    /// If the system settings are cached reload them
    ///
    /// Returns ok if implemented
    fn reload_system_config(&self) -> Result<(), ()> {
        Err(())
    }
}

/// An incomplete set of recored types to resolve
pub enum QueryType {
    AAAA,
    A,
    MX,
    TXT,
}

/// An incomplete set of the results a typical, mobile client may request
pub enum Record {
    /// AAAA or A single IpAddr result
    IpAddr(IpAddr),
    /// Many mail records
    MX(Vec<PriorityEntry<IpAddr>>),
    /// Many TXT records
    TXT(Vec<String>),
}

pub enum ResolveError {
    /// Some lower stack Input/Output Error
    IO(std::io::Error),
    /// Not found
    NotResolved,
}

pub struct PriorityEntry<T> {
    /// TODO check RFCs for the actual datatype
    pub priority: isize,
    pub value: T,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
