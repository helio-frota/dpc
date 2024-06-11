# Duplicrabs

### crab 0

> [!TIP]
> Exactly the same

```rust
    fn limiting(
        self,
        db: &'db C,
        offset: u64,
        limit: u64,
    ) -> Limiter<'db, C, Self::FetchSelector, Self::CountSelector> {
        let selector = self
            .clone()
            .limit(NonZeroU64::new(limit).map(|limit| limit.get()))
            .offset(NonZeroU64::new(offset).map(|offset| offset.get()))
            .into_model();
```

`"trustify/common/src/db/limiter.rs"
`
```rust
    fn limiting(
        self,
        db: &'db C,
        offset: u64,
        limit: u64,
    ) -> Limiter<'db, C, Self::FetchSelector, Self::CountSelector> {
        let selector = self
            .clone()
            .limit(NonZeroU64::new(limit).map(|limit| limit.get()))
            .offset(NonZeroU64::new(offset).map(|offset| offset.get()))
            .into_model();
```

`"trustify/common/src/db/limiter.rs"
`
### crab 1

> [!WARNING]
> Almost the same

```rust
impl Display for AttackVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Network => 'N',
                Self::Adjacent => 'A',
                Self::Local => 'L',
                Self::Physical => 'P',
            }
        )
    }
}
```

`"trustify/cvss/src/cvss3/mod.rs"
`
```rust
impl Display for AttackVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Network => 'N',
                Self::Adjacent => 'A',
                Self::Local => 'L',
                Self::Physical => 'P',
            }
        )
    }
}
```

`"trustify/cvss/src/cvss4.rs"
`
### crab 2

> [!WARNING]
> Almost the same

```rust
impl Display for AttackComplexity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Low => 'L',
                Self::High => 'H',
            }
        )
    }
}
```

`"trustify/cvss/src/cvss3/mod.rs"
`
```rust
impl Display for AttackComplexity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Low => 'L',
                Self::High => 'H',
            }
        )
    }
}
```

`"trustify/cvss/src/cvss4.rs"
`
### crab 3

> [!WARNING]
> Almost the same

```rust
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("AC:") {
            return Err(Self::Err::AttackComplexity);
        }
        match s.chars().nth(3) {
            Some('L') => Ok(Self::Low),
            Some('H') => Ok(Self::High),
            _ => Err(Self::Err::AttackComplexity),
        }
    }
}
```

`"trustify/cvss/src/cvss3/mod.rs"
`
```rust
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("AC:") {
            return Err(Self::Err::AttackComplexity);
        }
        match s.chars().nth(3) {
            Some('L') => Ok(Self::Low),
            Some('H') => Ok(Self::High),
            _ => Err(Self::Err::AttackComplexity),
        }
    }
}
```

`"trustify/cvss/src/cvss4.rs"
`
### crab 4

> [!WARNING]
> Almost the same

```rust
impl Display for PrivilegesRequired {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => 'N',
                Self::Low => 'L',
                Self::High => 'H',
            }
        )
    }
}
```

`"trustify/cvss/src/cvss3/mod.rs"
`
```rust
impl Display for PrivilegesRequired {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => 'N',
                Self::Low => 'L',
                Self::High => 'H',
            }
        )
    }
}
```

`"trustify/cvss/src/cvss4.rs"
`
### crab 5

> [!WARNING]
> Almost the same

```rust
        match s.chars().nth(3) {
            Some('N') => Ok(Self::None),
            Some('L') => Ok(Self::Low),
            Some('H') => Ok(Self::High),
            _ => Err(Self::Err::VulnerableIntegrity),
        }
    }
}
```

`"trustify/cvss/src/cvss4.rs"
`
```rust
        match s.chars().nth(3) {
            Some('N') => Ok(Self::None),
            Some('L') => Ok(Self::Low),
            Some('H') => Ok(Self::High),
            _ => Err(Self::Err::VulnerableAvailability),
        }
    }
}
```

`"trustify/cvss/src/cvss4.rs"
`
### crab 6

> [!WARNING]
> Almost the same

```rust
        match s.chars().nth(3) {
            Some('N') => Ok(Self::None),
            Some('L') => Ok(Self::Low),
            Some('H') => Ok(Self::High),
            _ => Err(Self::Err::SubsequentIntegrity),
        }
    }
}
```

`"trustify/cvss/src/cvss4.rs"
`
```rust
        match s.chars().nth(3) {
            Some('N') => Ok(Self::None),
            Some('L') => Ok(Self::Low),
            Some('H') => Ok(Self::High),
            _ => Err(Self::Err::SubsequentAvailability),
        }
    }
}
```

`"trustify/cvss/src/cvss4.rs"
`
### crab 7

> [!WARNING]
> Almost the same

```rust
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::advisory::Entity",
    from = "super::cvss4::Column::AdvisoryId"
    to = "super::advisory::Column::Id")]
    Advisory,
```

`"trustify/entity/src/cvss4.rs"
`
```rust
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::advisory::Entity",
    from = "super::cvss3::Column::AdvisoryId"
    to = "super::advisory::Column::Id")]
    Advisory,
```

`"trustify/entity/src/cvss3.rs"
`
### crab 8

> [!WARNING]
> Almost the same

```rust
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "cvss4_av")]
pub enum AttackVector {
    #[sea_orm(string_value = "n")]
    Network,
    #[sea_orm(string_value = "a")]
    Adjacent,
    #[sea_orm(string_value = "l")]
    Local,
    #[sea_orm(string_value = "p")]
    Physical,
}
```

`"trustify/entity/src/cvss4.rs"
`
```rust
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "cvss3_av")]
pub enum AttackVector {
    #[sea_orm(string_value = "n")]
    Network,
    #[sea_orm(string_value = "a")]
    Adjacent,
    #[sea_orm(string_value = "l")]
    Local,
    #[sea_orm(string_value = "p")]
    Physical,
}
```

`"trustify/entity/src/cvss3.rs"
`
### crab 9

> [!WARNING]
> Almost the same

```rust
impl From<AttackVector> for cvss4::AttackVector {
    fn from(value: AttackVector) -> Self {
        match value {
            AttackVector::Network => Self::Network,
            AttackVector::Adjacent => Self::Adjacent,
            AttackVector::Local => Self::Local,
            AttackVector::Physical => Self::Physical,
        }
    }
}
```

`"trustify/entity/src/cvss4.rs"
`
```rust
impl From<AttackVector> for cvss3::AttackVector {
    fn from(value: AttackVector) -> Self {
        match value {
            AttackVector::Network => Self::Network,
            AttackVector::Adjacent => Self::Adjacent,
            AttackVector::Local => Self::Local,
            AttackVector::Physical => Self::Physical,
        }
    }
}
```

`"trustify/entity/src/cvss3.rs"
`
### crab 10

> [!WARNING]
> Almost the same

```rust
impl From<cvss4::AttackVector> for AttackVector {
    fn from(value: cvss4::AttackVector) -> Self {
        match value {
            cvss4::AttackVector::Network => Self::Network,
            cvss4::AttackVector::Adjacent => Self::Adjacent,
            cvss4::AttackVector::Local => Self::Local,
            cvss4::AttackVector::Physical => Self::Physical,
        }
    }
}
```

`"trustify/entity/src/cvss4.rs"
`
```rust
impl From<cvss3::AttackVector> for AttackVector {
    fn from(value: cvss3::AttackVector) -> Self {
        match value {
            cvss3::AttackVector::Network => Self::Network,
            cvss3::AttackVector::Adjacent => Self::Adjacent,
            cvss3::AttackVector::Local => Self::Local,
            cvss3::AttackVector::Physical => Self::Physical,
        }
    }
}
```

`"trustify/entity/src/cvss3.rs"
`
### crab 11

> [!WARNING]
> Almost the same

```rust
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "cvss4_ac")]
pub enum AttackComplexity {
    #[sea_orm(string_value = "l")]
    Low,
    #[sea_orm(string_value = "h")]
    High,
}
```

`"trustify/entity/src/cvss4.rs"
`
```rust
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "cvss3_ac")]
pub enum AttackComplexity {
    #[sea_orm(string_value = "l")]
    Low,
    #[sea_orm(string_value = "h")]
    High,
}
```

`"trustify/entity/src/cvss3.rs"
`
### crab 12

> [!WARNING]
> Almost the same

```rust
impl From<AttackComplexity> for cvss4::AttackComplexity {
    fn from(value: AttackComplexity) -> Self {
        match value {
            AttackComplexity::Low => Self::Low,
            AttackComplexity::High => Self::High,
        }
    }
}
```

`"trustify/entity/src/cvss4.rs"
`
```rust
impl From<AttackComplexity> for cvss3::AttackComplexity {
    fn from(value: AttackComplexity) -> Self {
        match value {
            AttackComplexity::Low => Self::Low,
            AttackComplexity::High => Self::High,
        }
    }
}
```

`"trustify/entity/src/cvss3.rs"
`
### crab 13

> [!WARNING]
> Almost the same

```rust
impl From<cvss4::AttackComplexity> for AttackComplexity {
    fn from(value: cvss4::AttackComplexity) -> Self {
        match value {
            cvss4::AttackComplexity::Low => Self::Low,
            cvss4::AttackComplexity::High => Self::High,
        }
    }
}
```

`"trustify/entity/src/cvss4.rs"
`
```rust
impl From<cvss3::AttackComplexity> for AttackComplexity {
    fn from(value: cvss3::AttackComplexity) -> Self {
        match value {
            cvss3::AttackComplexity::Low => Self::Low,
            cvss3::AttackComplexity::High => Self::High,
        }
    }
}
```

`"trustify/entity/src/cvss3.rs"
`
### crab 14

> [!WARNING]
> Almost the same

```rust
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "cvss4_pr")]
pub enum PrivilegesRequired {
    #[sea_orm(string_value = "n")]
    None,
    #[sea_orm(string_value = "l")]
    Low,
    #[sea_orm(string_value = "h")]
    High,
}
```

`"trustify/entity/src/cvss4.rs"
`
```rust
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "cvss3_pr")]
pub enum PrivilegesRequired {
    #[sea_orm(string_value = "n")]
    None,
    #[sea_orm(string_value = "l")]
    Low,
    #[sea_orm(string_value = "h")]
    High,
}
```

`"trustify/entity/src/cvss3.rs"
`
### crab 15

> [!WARNING]
> Almost the same

```rust
impl From<PrivilegesRequired> for cvss4::PrivilegesRequired {
    fn from(value: PrivilegesRequired) -> Self {
        match value {
            PrivilegesRequired::None => Self::None,
            PrivilegesRequired::Low => Self::Low,
            PrivilegesRequired::High => Self::High,
        }
    }
}
```

`"trustify/entity/src/cvss4.rs"
`
```rust
impl From<PrivilegesRequired> for cvss3::PrivilegesRequired {
    fn from(value: PrivilegesRequired) -> Self {
        match value {
            PrivilegesRequired::None => Self::None,
            PrivilegesRequired::Low => Self::Low,
            PrivilegesRequired::High => Self::High,
        }
    }
}
```

`"trustify/entity/src/cvss3.rs"
`
### crab 16

> [!WARNING]
> Almost the same

```rust
impl From<cvss4::PrivilegesRequired> for PrivilegesRequired {
    fn from(value: cvss4::PrivilegesRequired) -> Self {
        match value {
            cvss4::PrivilegesRequired::None => Self::None,
            cvss4::PrivilegesRequired::Low => Self::Low,
            cvss4::PrivilegesRequired::High => Self::High,
        }
    }
}
```

`"trustify/entity/src/cvss4.rs"
`
```rust
impl From<cvss3::PrivilegesRequired> for PrivilegesRequired {
    fn from(value: cvss3::PrivilegesRequired) -> Self {
        match value {
            cvss3::PrivilegesRequired::None => Self::None,
            cvss3::PrivilegesRequired::Low => Self::Low,
            cvss3::PrivilegesRequired::High => Self::High,
        }
    }
}
```

`"trustify/entity/src/cvss3.rs"
`
### crab 17

> [!WARNING]
> Almost the same

```rust
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Cvss3Av::Cvss3Av)
                    .values([Cvss3Av::N, Cvss3Av::A, Cvss3Av::L, Cvss3Av::P])
                    .to_owned(),
            )
            .await?;
```

`"trustify/migration/src/m0000010_create_cvss3_enums.rs"
`
```rust
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Cvss4Av::Cvss4Av)
                    .values([Cvss4Av::N, Cvss4Av::A, Cvss4Av::L, Cvss4Av::P])
                    .to_owned(),
            )
            .await?;
```

`"trustify/migration/src/m0000020_create_cvss4_enums.rs"
`
### crab 18

> [!WARNING]
> Almost the same

```rust
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cvss3::Table).if_exists().to_owned())
            .await
    }
}
```

`"trustify/migration/src/m0000070_create_cvss3.rs"
`
```rust
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cvss4::Table).if_exists().to_owned())
            .await
    }
}
```

`"trustify/migration/src/m0000080_create_cvss4.rs"
`
### crab 19

> [!TIP]
> Exactly the same

```rust
        manager
            .alter_table(
                Table::alter()
                    .table(AffectedPackageVersionRange::Table)
                    .drop_column(AffectedPackageVersionRange::AdvisoryId)
                    .to_owned(),
            )
            .await?;
```

`"trustify/migration/src/m0000310_alter_advisory_primary_key.rs"
`
```rust
        manager
            .alter_table(
                Table::alter()
                    .table(AffectedPackageVersionRange::Table)
                    .drop_column(AffectedPackageVersionRange::AdvisoryId)
                    .to_owned(),
            )
            .await?;
```

`"trustify/migration/src/m0000310_alter_advisory_primary_key.rs"
`
### crab 20

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
### crab 21

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
### crab 22

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 23

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 24

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 25

> [!WARNING]
> Almost the same

```rust
    graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
### crab 26

> [!WARNING]
> Almost the same

```rust
    graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
### crab 27

> [!TIP]
> Exactly the same

```rust
    let app = actix_web::test::init_service(
        App::new().service(
            web::scope("/api")
                .configure(|config| crate::endpoints::configure(config, db, storage.clone())),
        ),
    )
    .await;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    let app = actix_web::test::init_service(
        App::new().service(
            web::scope("/api")
                .configure(|config| crate::endpoints::configure(config, db, storage.clone())),
        ),
    )
    .await;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
### crab 28

> [!TIP]
> Exactly the same

```rust
    graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: Some("Red Hat Product Security".to_string()),
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: Some("Red Hat Product Security".to_string()),
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
### crab 29

> [!TIP]
> Exactly the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: Some("Red Hat Product Security".to_string()),
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: Some("Red Hat Product Security".to_string()),
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
### crab 30

> [!TIP]
> Exactly the same

```rust
    let advisory_vuln = advisory
        .link_to_vulnerability("CVE-123", None, Transactional::None)
        .await?;
    advisory_vuln
        .ingest_cvss3_score(
            Cvss3Base {
                minor_version: 0,
                av: AttackVector::Network,
                ac: AttackComplexity::Low,
                pr: PrivilegesRequired::High,
                ui: UserInteraction::None,
                s: Scope::Changed,
                c: Confidentiality::High,
                i: Integrity::None,
                a: Availability::None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    let advisory_vuln = advisory
        .link_to_vulnerability("CVE-123", None, Transactional::None)
        .await?;
    advisory_vuln
        .ingest_cvss3_score(
            Cvss3Base {
                minor_version: 0,
                av: AttackVector::Network,
                ac: AttackComplexity::Low,
                pr: PrivilegesRequired::High,
                ui: UserInteraction::None,
                s: Scope::Changed,
                c: Confidentiality::High,
                i: Integrity::None,
                a: Availability::None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
### crab 31

> [!WARNING]
> Almost the same

```rust
    let advisory_vuln = advisory
        .link_to_vulnerability("CVE-123", None, Transactional::None)
        .await?;
    advisory_vuln
        .ingest_cvss3_score(
            Cvss3Base {
                minor_version: 0,
                av: AttackVector::Network,
                ac: AttackComplexity::Low,
                pr: PrivilegesRequired::High,
                ui: UserInteraction::None,
                s: Scope::Changed,
                c: Confidentiality::High,
                i: Integrity::None,
                a: Availability::None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    let advisory_vuln = advisory
        .link_to_vulnerability("CVE-123", None, Transactional::None)
        .await?;
    advisory_vuln
        .ingest_cvss3_score(
            Cvss3Base {
                minor_version: 0,
                av: AttackVector::Network,
                ac: AttackComplexity::Low,
                pr: PrivilegesRequired::High,
                ui: UserInteraction::None,
                s: Scope::Changed,
                c: Confidentiality::High,
                i: Integrity::None,
                a: Availability::None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 32

> [!WARNING]
> Almost the same

```rust
    let advisory_vuln = advisory
        .link_to_vulnerability("CVE-123", None, Transactional::None)
        .await?;
    advisory_vuln
        .ingest_cvss3_score(
            Cvss3Base {
                minor_version: 0,
                av: AttackVector::Network,
                ac: AttackComplexity::Low,
                pr: PrivilegesRequired::High,
                ui: UserInteraction::None,
                s: Scope::Changed,
                c: Confidentiality::High,
                i: Integrity::None,
                a: Availability::None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    let advisory_vuln = advisory
        .link_to_vulnerability("CVE-123", None, Transactional::None)
        .await?;
    advisory_vuln
        .ingest_cvss3_score(
            Cvss3Base {
                minor_version: 0,
                av: AttackVector::Network,
                ac: AttackComplexity::Low,
                pr: PrivilegesRequired::High,
                ui: UserInteraction::None,
                s: Scope::Changed,
                c: Confidentiality::High,
                i: Integrity::None,
                a: Availability::None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 33

> [!WARNING]
> Almost the same

```rust
    let response = actix_web::test::call_service(&app, request).await;
    assert!(response.status().is_success());
    let id: String = actix_web::test::read_body_json(response).await;
    assert_eq!(id, "RUSTSEC-2021-0079");
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
```rust
    let response = actix_web::test::call_service(&app, request).await;
    assert!(response.status().is_success());
    let id: String = actix_web::test::read_body_json(response).await;
    assert_eq!(id, "CVE-2024-27088");
```

`"trustify/modules/fundamental/src/advisory/endpoints/test.rs"
`
### crab 34

> [!TIP]
> Exactly the same

```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/advisory/model/details/advisory_vulnerability.rs"
`
```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/advisory/model/details/advisory_vulnerability.rs"
`
### crab 35

> [!WARNING]
> Almost the same

```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/advisory/model/details/advisory_vulnerability.rs"
`
```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/vulnerability/model/details/vulnerability_advisory.rs"
`
### crab 36

> [!WARNING]
> Almost the same

```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/advisory/model/details/advisory_vulnerability.rs"
`
```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/vulnerability/model/details/vulnerability_advisory.rs"
`
### crab 37

> [!WARNING]
> Almost the same

```rust
                packages
                    .drain(..)
                    .zip(package_version_ranges.drain(..))
                    .for_each(|(package, version_range)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/advisory/model/details/advisory_vulnerability.rs"
`
```rust
                packages
                    .drain(..)
                    .zip(package_version_ranges.drain(..))
                    .for_each(|(package, version_range)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/vulnerability/model/details/vulnerability_advisory.rs"
`
### crab 38

> [!WARNING]
> Almost the same

```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/advisory/model/details/advisory_vulnerability.rs"
`
```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/vulnerability/model/details/vulnerability_advisory.rs"
`
### crab 39

> [!WARNING]
> Almost the same

```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/advisory/model/details/advisory_vulnerability.rs"
`
```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/vulnerability/model/details/vulnerability_advisory.rs"
`
### crab 40

> [!TIP]
> Exactly the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
### crab 41

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 42

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 43

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 44

> [!TIP]
> Exactly the same

```rust
    let advisory_vuln = advisory
        .link_to_vulnerability("CVE-123", None, Transactional::None)
        .await?;
    advisory_vuln
        .ingest_cvss3_score(
            Cvss3Base {
                minor_version: 0,
                av: AttackVector::Network,
                ac: AttackComplexity::Low,
                pr: PrivilegesRequired::None,
                ui: UserInteraction::None,
                s: Scope::Unchanged,
                c: Confidentiality::None,
                i: Integrity::High,
                a: Availability::High,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
```rust
    let advisory_vuln = advisory
        .link_to_vulnerability("CVE-123", None, Transactional::None)
        .await?;
    advisory_vuln
        .ingest_cvss3_score(
            Cvss3Base {
                minor_version: 0,
                av: AttackVector::Network,
                ac: AttackComplexity::Low,
                pr: PrivilegesRequired::None,
                ui: UserInteraction::None,
                s: Scope::Unchanged,
                c: Confidentiality::None,
                i: Integrity::High,
                a: Availability::High,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
### crab 45

> [!TIP]
> Exactly the same

```rust
    graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
```rust
    graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
### crab 46

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 47

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 48

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/advisory/service/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 49

> [!WARNING]
> Almost the same

```rust
#[test_context(TrustifyContext, skip_teardown)]
#[test(actix_web::test)]
async fn types(ctx: TrustifyContext) -> Result<(), anyhow::Error> {
    let db = ctx.db;
```

`"trustify/modules/fundamental/src/package/endpoints/test.rs"
`
```rust
#[test_context(TrustifyContext, skip_teardown)]
#[test(actix_web::test)]
async fn r#type(ctx: TrustifyContext) -> Result<(), anyhow::Error> {
    let db = ctx.db;
```

`"trustify/modules/fundamental/src/package/endpoints/test.rs"
`
### crab 50

> [!WARNING]
> Almost the same

```rust
#[test_context(TrustifyContext, skip_teardown)]
#[test(actix_web::test)]
async fn types(ctx: TrustifyContext) -> Result<(), anyhow::Error> {
    let db = ctx.db;
```

`"trustify/modules/fundamental/src/package/endpoints/test.rs"
`
```rust
#[test_context(TrustifyContext, skip_teardown)]
#[test(actix_web::test)]
async fn base(ctx: TrustifyContext) -> Result<(), anyhow::Error> {
    let db = ctx.db;
```

`"trustify/modules/fundamental/src/package/endpoints/test.rs"
`
### crab 51

> [!TIP]
> Exactly the same

```rust
    let (namespace, name) = if let Some((namespace, name)) = namespace_and_name.split_once('/') {
        (Some(namespace.to_string()), name.to_string())
    } else {
        (None, namespace_and_name)
    };
```

`"trustify/modules/fundamental/src/package/endpoints/type.rs"
`
```rust
    let (namespace, name) = if let Some((namespace, name)) = namespace_and_name.split_once('/') {
        (Some(namespace.to_string()), name.to_string())
    } else {
        (None, namespace_and_name)
    };
```

`"trustify/modules/fundamental/src/package/endpoints/type.rs"
`
### crab 52

> [!TIP]
> Exactly the same

```rust
        if let Some(ns) = namespace {
            query = query.filter(package::Column::Namespace.eq(ns));
        } else {
            query = query.filter(package::Column::Namespace.is_null());
        }
```

`"trustify/modules/fundamental/src/package/service/mod.rs"
`
```rust
        if let Some(ns) = namespace {
            query = query.filter(package::Column::Namespace.eq(ns));
        } else {
            query = query.filter(package::Column::Namespace.is_null());
        }
```

`"trustify/modules/fundamental/src/package/service/mod.rs"
`
### crab 53

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 54

> [!TIP]
> Exactly the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 55

> [!WARNING]
> Almost the same

```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-1",
            "http://redhat.com/",
            "8675309",
            AdvisoryInformation {
                title: Some("RHSA-1".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
```rust
    let advisory = graph
        .ingest_advisory(
            "RHSA-2",
            "http://redhat.com/",
            "8675319",
            AdvisoryInformation {
                title: Some("RHSA-2".to_string()),
                issuer: None,
                published: Some(OffsetDateTime::now_utc()),
                modified: None,
                withdrawn: None,
            },
            (),
        )
        .await?;
```

`"trustify/modules/fundamental/src/vulnerability/endpoints/test.rs"
`
### crab 56

> [!TIP]
> Exactly the same

```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/vulnerability/model/details/vulnerability_advisory.rs"
`
```rust
                packages.drain(..).zip(package_versions.drain(..)).for_each(
                    |(package, version)| {
                        if let Some(package) = package {
                            let package_assertions = assertions
                                .entry(
                                    Purl {
                                        ty: package.r#type,
                                        namespace: package.namespace,
                                        name: package.name,
                                        version: None,
                                        qualifiers: Default::default(),
                                    }
                                    .to_string(),
                                )
                                .or_insert(vec![]);
```

`"trustify/modules/fundamental/src/vulnerability/model/details/vulnerability_advisory.rs"
`
### crab 57

> [!WARNING]
> Almost the same

```rust
        if let Err(err) = &result {
            match err {
                StorageError::Validation(ValidationError::Retrieval(
                    RetrievalError::InvalidResponse { code, .. },
                )) => {
                    self.0.report.lock().add_error(
                        Phase::Retrieval,
                        file,
                        Severity::Error,
                        format!("retrieval of document failed: {code}"),
                    );
```

`"trustify/modules/importer/src/server/csaf/report.rs"
`
```rust
        if let Err(err) = &result {
            match err {
                StorageError::Validation(ValidationError::Retrieval(
                    RetrievalError::InvalidResponse { code, .. },
                )) => {
                    self.0.report.lock().add_error(
                        Phase::Retrieval,
                        file,
                        Severity::Error,
                        format!("retrieval of document failed: {code}"),
                    );
```

`"trustify/modules/importer/src/server/sbom/report.rs"
`
### crab 58

> [!WARNING]
> Almost the same

```rust
                    if code.is_client_error() {
                        // If it's a client error, there's no need to re-try. We simply claim
                        // success after we logged it.
                        return Ok(());
                    }
                }
                StorageError::Validation(ValidationError::DigestMismatch {
                    expected,
                    actual,
                    ..
                }) => {
                    self.0.report.lock().add_error(
                        Phase::Validation,
                        file,
                        Severity::Error,
                        format!("digest mismatch - expected: {expected}, actual: {actual}"),
                    );
```

`"trustify/modules/importer/src/server/csaf/report.rs"
`
```rust
                    if code.is_client_error() {
                        // If it's a client error, there's no need to re-try. We simply claim
                        // success after we logged it.
                        return Ok(());
                    }
                }
                StorageError::Validation(ValidationError::DigestMismatch {
                    expected,
                    actual,
                    ..
                }) => {
                    self.0.report.lock().add_error(
                        Phase::Validation,
                        file,
                        Severity::Error,
                        format!("digest mismatch - expected: {expected}, actual: {actual}"),
                    );
```

`"trustify/modules/importer/src/server/sbom/report.rs"
`
### crab 59

> [!WARNING]
> Almost the same

```rust
                    return Ok(());
                }
                StorageError::Validation(ValidationError::Signature { error, .. }) => {
                    self.0.report.lock().add_error(
                        Phase::Validation,
                        file,
                        Severity::Error,
                        format!("unable to verify signature: {error}"),
                    );
```

`"trustify/modules/importer/src/server/csaf/report.rs"
`
```rust
                    return Ok(());
                }
                StorageError::Validation(ValidationError::Signature { error, .. }) => {
                    self.0.report.lock().add_error(
                        Phase::Validation,
                        file,
                        Severity::Error,
                        format!("unable to verify signature: {error}"),
                    );
```

`"trustify/modules/importer/src/server/sbom/report.rs"
`
### crab 60

> [!WARNING]
> Almost the same

```rust
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    Storage(anyhow::Error),
}
```

`"trustify/modules/importer/src/server/csaf/storage.rs"
`
```rust
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    Storage(anyhow::Error),
}
```

`"trustify/modules/importer/src/server/sbom/storage.rs"
`
### crab 61

> [!WARNING]
> Almost the same

```rust
        Walker::new(source)
            .walk(filter)
            .await
            // if the walker fails, we record the outcome as part of the report, but skip any
            // further processing, like storing the marker
            .map_err(|err| ScannerError::Normal {
                err: err.into(),
                output: RunOutput {
                    report: report.lock().clone().build(),
                    continuation: None,
                },
            })?;
```

`"trustify/modules/importer/src/server/csaf/mod.rs"
`
```rust
        Walker::new(source)
            .walk(filter)
            .await
            // if the walker fails, we record the outcome as part of the report, but skip any
            // further processing, like storing the marker
            .map_err(|err| ScannerError::Normal {
                err: err.into(),
                output: RunOutput {
                    report: report.lock().clone().build(),
                    continuation: None,
                },
            })?;
```

`"trustify/modules/importer/src/server/sbom/mod.rs"
`
### crab 62

> [!WARNING]
> Almost the same

```rust
impl Callbacks for Context {
    fn loading_error(&mut self, path: PathBuf, message: String) {
        self.report.lock().add_error(
            Phase::Validation,
            path.to_string_lossy(),
            Severity::Error,
            message,
        );
    }
```

`"trustify/modules/importer/src/server/cve/mod.rs"
`
```rust
impl Callbacks for Context {
    fn loading_error(&mut self, path: PathBuf, message: String) {
        self.report.lock().add_error(
            Phase::Validation,
            path.to_string_lossy(),
            Severity::Error,
            message,
        );
    }
```

`"trustify/modules/importer/src/server/osv/mod.rs"
`
### crab 63

> [!WARNING]
> Almost the same

```rust
        let continuation = match working_dir {
            Some(working_dir) => walker.working_dir(working_dir).run().await,
            None => walker.run().await,
        }
        .map_err(|err| ScannerError::Critical(err.into()))?;
```

`"trustify/modules/importer/src/server/cve/mod.rs"
`
```rust
        let continuation = match working_dir {
            Some(working_dir) => walker.working_dir(working_dir).run().await,
            None => walker.run().await,
        }
        .map_err(|err| ScannerError::Critical(err.into()))?;
```

`"trustify/modules/importer/src/server/osv/mod.rs"
`
### crab 64

> [!WARNING]
> Almost the same

```rust
pub trait Callbacks: Send + 'static {
    /// Handle an error while loading the file
    #[allow(unused)]
    fn loading_error(&mut self, path: PathBuf, message: String) {}
```

`"trustify/modules/importer/src/server/cve/walker.rs"
`
```rust
pub trait Callbacks: Send + 'static {
    /// Handle an error while loading the file
    #[allow(unused)]
    fn loading_error(&mut self, path: PathBuf, message: String) {}
```

`"trustify/modules/importer/src/server/osv/walker.rs"
`
### crab 65

> [!TIP]
> Exactly the same

```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("baz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("baz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
### crab 66

> [!WARNING]
> Almost the same

```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("baz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
### crab 67

> [!WARNING]
> Almost the same

```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("baz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
### crab 68

> [!WARNING]
> Almost the same

```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("baz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
### crab 69

> [!WARNING]
> Almost the same

```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("baz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
### crab 70

> [!WARNING]
> Almost the same

```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("baz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
### crab 71

> [!WARNING]
> Almost the same

```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("baz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
### crab 72

> [!WARNING]
> Almost the same

```rust
    let req = actix::TestRequest::put()
        .uri("/api/v1/importer/foo")
        .set_json(mock_configuration("buz"))
        .append_header((header::IF_MATCH, etag.clone()))
        .to_request();
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let req = actix::TestRequest::put()
        .uri("/api/v1/importer/foo")
        .set_json(mock_configuration("boz"))
        .append_header((header::IF_MATCH, etag.clone()))
        .to_request();
```

`"trustify/modules/importer/src/test.rs"
`
### crab 73

> [!WARNING]
> Almost the same

```rust
    let req = actix::TestRequest::put()
        .uri("/api/v1/importer/foo")
        .set_json(mock_configuration("buz"))
        .append_header((header::IF_MATCH, etag.clone()))
        .to_request();
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let req = actix::TestRequest::put()
        .uri("/api/v1/importer/foo2")
        .set_json(mock_configuration("boz"))
        .append_header((header::IF_MATCH, etag.clone()))
        .to_request();
```

`"trustify/modules/importer/src/test.rs"
`
### crab 74

> [!TIP]
> Exactly the same

```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
### crab 75

> [!TIP]
> Exactly the same

```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
### crab 76

> [!WARNING]
> Almost the same

```rust
    let req = actix::TestRequest::put()
        .uri("/api/v1/importer/foo")
        .set_json(mock_configuration("boz"))
        .append_header((header::IF_MATCH, etag.clone()))
        .to_request();
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let req = actix::TestRequest::put()
        .uri("/api/v1/importer/foo2")
        .set_json(mock_configuration("boz"))
        .append_header((header::IF_MATCH, etag.clone()))
        .to_request();
```

`"trustify/modules/importer/src/test.rs"
`
### crab 77

> [!TIP]
> Exactly the same

```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
```rust
    let result: Importer = actix::read_body_json(resp).await;
    assert_eq!(
        result,
        Importer {
            name: "foo".into(),
            data: ImporterData {
                configuration: mock_configuration("buz"),
                state: State::Waiting,
                last_change: result.data.last_change, // we can't predict timestamps
                last_success: None,
                last_error: None,
                last_run: None,
                continuation: serde_json::Value::Null,
            }
        }
    );
```

`"trustify/modules/importer/src/test.rs"
`
### crab 78

> [!WARNING]
> Almost the same

```rust
#[cfg(test)]
mod test {
    use crate::graph::Graph;
    use test_context::test_context;
    use test_log::test;
    use trustify_common::db::{test::TrustifyContext, Transactional};
```

`"trustify/modules/ingestor/src/graph/advisory/mod.rs"
`
```rust
#[cfg(test)]
mod test {
    use crate::graph::Graph;
    use test_context::test_context;
    use test_log::test;
    use trustify_common::db::{test::TrustifyContext, Transactional};
```

`"trustify/modules/ingestor/src/service/cve/loader.rs"
`
### crab 79

> [!WARNING]
> Almost the same

```rust
    #[test_context(TrustifyContext, skip_teardown)]
    #[test(tokio::test)]
    async fn ingest_advisories(ctx: TrustifyContext) -> Result<(), anyhow::Error> {
        let db = ctx.db;
        let system = Graph::new(db);
```

`"trustify/modules/ingestor/src/graph/advisory/mod.rs"
`
```rust
    #[test_context(TrustifyContext, skip_teardown)]
    #[test(tokio::test)]
    async fn ingest_advisory_cve(ctx: TrustifyContext) -> Result<(), anyhow::Error> {
        let db = ctx.db;
        let system = Graph::new(db);
```

`"trustify/modules/ingestor/src/graph/advisory/mod.rs"
`
### crab 80

> [!WARNING]
> Almost the same

```rust
        let affected1 = advisory_vulnerability
            .ingest_affected_package_range(
                &"pkg://maven/io.quarkus/quarkus-core".try_into()?,
                "1.0.2",
                "1.2.0",
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/advisory/mod.rs"
`
```rust
        let affected2 = advisory_vulnerability
            .ingest_affected_package_range(
                &"pkg://maven/io.quarkus/quarkus-core".try_into()?,
                "1.0.2",
                "1.2.0",
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/advisory/mod.rs"
`
### crab 81

> [!WARNING]
> Almost the same

```rust
        let affected1 = advisory_vulnerability
            .ingest_affected_package_range(
                &"pkg://maven/io.quarkus/quarkus-core".try_into()?,
                "1.0.2",
                "1.2.0",
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/advisory/mod.rs"
`
```rust
        let _affected = advisory_vulnerability
            .ingest_affected_package_range(
                &"pkg://maven/io.quarkus/quarkus-core".try_into()?,
                "1.0.2",
                "1.2.0",
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/advisory/mod.rs"
`
### crab 82

> [!WARNING]
> Almost the same

```rust
        let affected2 = advisory_vulnerability
            .ingest_affected_package_range(
                &"pkg://maven/io.quarkus/quarkus-core".try_into()?,
                "1.0.2",
                "1.2.0",
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/advisory/mod.rs"
`
```rust
        let _affected = advisory_vulnerability
            .ingest_affected_package_range(
                &"pkg://maven/io.quarkus/quarkus-core".try_into()?,
                "1.0.2",
                "1.2.0",
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/advisory/mod.rs"
`
### crab 83

> [!WARNING]
> Almost the same

```rust
        let fixed1 = advisory_vulnerability
            .ingest_fixed_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2.0".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/advisory/mod.rs"
`
```rust
        let fixed2 = advisory_vulnerability
            .ingest_fixed_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2.0".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/advisory/mod.rs"
`
### crab 84

> [!WARNING]
> Almost the same

```rust
    pub async fn not_affected_assertions<TX: AsRef<Transactional>>(
        &self,
        tx: TX,
    ) -> Result<PackageVulnerabilityAssertions, Error> {
        #[derive(FromQueryResult, Debug)]
        struct NotAffectedVersion {
            version: String,
            identifier: String,
            location: String,
            sha256: String,
        }
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
```rust
    pub async fn not_affected_assertions<TX: AsRef<Transactional>>(
        &self,
        tx: TX,
    ) -> Result<PackageVulnerabilityAssertions, Error> {
        #[derive(FromQueryResult, Debug)]
        struct NotAffectedVersion {
            version: String,
            identifier: String,
            location: String,
            sha256: String,
        }
```

`"trustify/modules/ingestor/src/graph/purl/package_version.rs"
`
### crab 85

> [!WARNING]
> Almost the same

```rust
        let not_affected_versions = entity::not_affected_package_version::Entity::find()
            .column_as(entity::package_version::Column::Version, "version")
            .column_as(entity::advisory::Column::Id, "advisory_id")
            .column_as(entity::advisory::Column::Identifier, "identifier")
            .column_as(entity::advisory::Column::Location, "location")
            .column_as(entity::advisory::Column::Sha256, "sha256")
            .join(
                JoinType::Join,
                entity::not_affected_package_version::Relation::Advisory.def(),
            )
            .join(
                JoinType::Join,
                entity::not_affected_package_version::Relation::PackageVersion.def(),
            )
            .filter(entity::package_version::Column::PackageId.eq(self.package.id))
            .into_model::<NotAffectedVersion>()
            .all(&self.graph.connection(&tx))
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
```rust
        let not_affected_versions = entity::not_affected_package_version::Entity::find()
            .column_as(entity::package_version::Column::Version, "version")
            .column_as(entity::advisory::Column::Id, "advisory_id")
            .column_as(entity::advisory::Column::Identifier, "identifier")
            .column_as(entity::advisory::Column::Location, "location")
            .column_as(entity::advisory::Column::Sha256, "sha256")
            .join(
                JoinType::Join,
                entity::not_affected_package_version::Relation::Advisory.def(),
            )
            .join(
                JoinType::Join,
                entity::not_affected_package_version::Relation::PackageVersion.def(),
            )
            .filter(entity::package_version::Column::Id.eq(self.package_version.id))
            .into_model::<NotAffectedVersion>()
            .all(&self.package.graph.connection(&tx))
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/package_version.rs"
`
### crab 86

> [!WARNING]
> Almost the same

```rust
            assertions.assertions.push(Assertion::NotAffected {
                vulnerability,
                claimant: Claimant {
                    identifier: each.identifier,
                    location: each.location,
                    sha256: each.sha256,
                },
                version: each.version,
            })
        }
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
```rust
            assertions.assertions.push(Assertion::NotAffected {
                vulnerability,
                claimant: Claimant {
                    identifier: each.identifier,
                    location: each.location,
                    sha256: each.sha256,
                },
                version: each.version,
            });
        }
```

`"trustify/modules/ingestor/src/graph/purl/package_version.rs"
`
### crab 87

> [!WARNING]
> Almost the same

```rust
    #[test_context(TrustifyContext, skip_teardown)]
    #[test(tokio::test)]
    async fn ingest_package_versions(ctx: TrustifyContext) -> Result<(), anyhow::Error> {
        let db = ctx.db;
        let system = Graph::new(db);
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
```rust
    #[test_context(TrustifyContext, skip_teardown)]
    #[test(tokio::test)]
    async fn get_versions_paginated(ctx: TrustifyContext) -> Result<(), anyhow::Error> {
        let db = ctx.db;
        let system = Graph::new(db);
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
### crab 88

> [!TIP]
> Exactly the same

```rust
        redhat_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
```rust
        redhat_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
### crab 89

> [!WARNING]
> Almost the same

```rust
        redhat_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
```rust
        redhat_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/package_version.rs"
`
### crab 90

> [!TIP]
> Exactly the same

```rust
        ghsa_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
```rust
        ghsa_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
### crab 91

> [!WARNING]
> Almost the same

```rust
        ghsa_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
```rust
        ghsa_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/package_version.rs"
`
### crab 92

> [!TIP]
> Exactly the same

```rust
        redhat_advisory_vulnerability
            .ingest_affected_package_range(
                &"pkg://maven/io.quarkus/quarkus-core".try_into()?,
                "1.1",
                "1.3",
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
```rust
        redhat_advisory_vulnerability
            .ingest_affected_package_range(
                &"pkg://maven/io.quarkus/quarkus-core".try_into()?,
                "1.1",
                "1.3",
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
### crab 93

> [!WARNING]
> Almost the same

```rust
        redhat_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
```rust
        redhat_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/package_version.rs"
`
### crab 94

> [!WARNING]
> Almost the same

```rust
        ghsa_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/mod.rs"
`
```rust
        ghsa_advisory_vulnerability
            .ingest_not_affected_package_version(
                &"pkg://maven/io.quarkus/quarkus-core@1.2.2".try_into()?,
                Transactional::None,
            )
            .await?;
```

`"trustify/modules/ingestor/src/graph/purl/package_version.rs"
`
### crab 95

> [!WARNING]
> Almost the same

```rust
        let digests = reader.finish().map_err(|e| Error::Generic(e.into()))?;
        let encoded_sha256 = hex::encode(digests.sha256);
        if checksum != encoded_sha256 {
            return Err(Error::Storage(anyhow::Error::msg(
                "document integrity check failed",
            )));
        }
```

`"trustify/modules/ingestor/src/service/advisory/csaf/loader.rs"
`
```rust
        let digests = reader.finish().map_err(|e| Error::Generic(e.into()))?;
        let encoded_sha256 = hex::encode(digests.sha256);
        if checksum != encoded_sha256 {
            return Err(Error::Storage(anyhow::Error::msg(
                "document integrity check failed",
            )));
        }
```

`"trustify/modules/ingestor/src/service/cve/loader.rs"
`
### crab 96

> [!WARNING]
> Almost the same

```rust
    #[test_context(TrustifyContext, skip_teardown)]
    #[test(tokio::test)]
    async fn loader(ctx: TrustifyContext) -> Result<(), anyhow::Error> {
        let db = ctx.db;
        let graph = Graph::new(db);
```

`"trustify/modules/ingestor/src/service/advisory/csaf/loader.rs"
`
```rust
    #[test_context(TrustifyContext, skip_teardown)]
    #[test(tokio::test)]
    async fn loader(ctx: TrustifyContext) -> Result<(), anyhow::Error> {
        let db = ctx.db;
        let graph = Graph::new(db);
```

`"trustify/modules/ingestor/src/service/advisory/osv/loader.rs"
`
### crab 97

> [!WARNING]
> Almost the same

```rust
#[cfg(test)]
mod test {
    use crate::graph::Graph;
    use crate::service::{Format, IngestorService};
    use bytes::Bytes;
    use futures::stream;
    use std::convert::Infallible;
    use test_context::test_context;
    use test_log::test;
    use trustify_common::db::test::TrustifyContext;
    use trustify_module_storage::service::fs::FileSystemBackend;
```

`"trustify/modules/ingestor/src/service/sbom/cyclonedx.rs"
`
```rust
#[cfg(test)]
mod test {
    use crate::graph::Graph;
    use crate::service::{Format, IngestorService};
    use bytes::Bytes;
    use futures::stream;
    use std::convert::Infallible;
    use test_context::test_context;
    use test_log::test;
    use trustify_common::db::test::TrustifyContext;
    use trustify_module_storage::service::fs::FileSystemBackend;
```

`"trustify/modules/ingestor/src/service/sbom/spdx.rs"
`
### crab 98

> [!WARNING]
> Almost the same

```rust
        ingestor
            .ingest(
                "test",
                None,
                Format::sbom_from_bytes(data)?,
                stream::iter([Ok::<_, Infallible>(Bytes::from_static(data))]),
            )
            .await
            .expect("must ingest");
```

`"trustify/modules/ingestor/src/service/sbom/cyclonedx.rs"
`
```rust
        ingestor
            .ingest(
                "test",
                None,
                Format::sbom_from_bytes(data)?,
                stream::iter([Ok::<_, Infallible>(Bytes::from_static(data))]),
            )
            .await
            .expect("must ingest");
```

`"trustify/modules/ingestor/src/service/sbom/spdx.rs"
`
