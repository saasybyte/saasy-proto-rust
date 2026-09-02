use std::convert::{From, TryFrom};

use mediasoup::types::data_structures::{
    DtlsFingerprint as MediasoupDtlsFingerprint,
    DtlsRole as MediasoupDtlsRole,
};
use mediasoup::prelude::{
    DtlsParameters as MediasoupDtlsParameters,
    TransportId as MediasoupTransportId,
};

use crate::shared::{
    DtlsFingerprint,
    DtlsFingerprintAlgorithm,
    DtlsParameters,
    DtlsRole,
    TransportId,
};
use super::error::ConversionError;

impl TryFrom<DtlsRole> for MediasoupDtlsRole {
    type Error = ConversionError;

    fn try_from(role: DtlsRole) -> Result<Self, Self::Error> {
        match role {
            DtlsRole::Auto => Ok(Self::Auto),
            DtlsRole::Client => Ok(Self::Client),
            DtlsRole::Server => Ok(Self::Server),
            DtlsRole::Unspecified => Err(ConversionError::InvalidDtlsRole(role.into())),
        }
    }
}

impl From<MediasoupDtlsRole> for DtlsRole {
    fn from(role: MediasoupDtlsRole) -> Self {
        match role {
            MediasoupDtlsRole::Auto => Self::Auto,
            MediasoupDtlsRole::Client => Self::Client,
            MediasoupDtlsRole::Server => Self::Server,
        }
    }
}

impl TryFrom<DtlsFingerprint> for MediasoupDtlsFingerprint {
    type Error = ConversionError;

    fn try_from(fingerprint: DtlsFingerprint) -> Result<Self, Self::Error> {
        let value = fingerprint.value;
        let algorithm = DtlsFingerprintAlgorithm::try_from(fingerprint.algorithm)
            .map_err(|_| ConversionError::InvalidDtlsAlgorithm)?;

        match algorithm {
            DtlsFingerprintAlgorithm::Sha1 => {
                if value.len() != 20 {
                    return Err(ConversionError::InvalidFingerprintLength(20, value.len()));
                }
                let mut buf = [0u8; 20];
                buf.copy_from_slice(&value);
                Ok(Self::Sha1 { value: buf })
            }
            DtlsFingerprintAlgorithm::Sha224 => {
                if value.len() != 28 {
                    return Err(ConversionError::InvalidFingerprintLength(28, value.len()));
                }
                let mut buf = [0u8; 28];
                buf.copy_from_slice(&value);
                Ok(Self::Sha224 { value: buf })
            }
            DtlsFingerprintAlgorithm::Sha256 => {
                if value.len() != 32 {
                    return Err(ConversionError::InvalidFingerprintLength(32, value.len()));
                }
                let mut buf = [0u8; 32];
                buf.copy_from_slice(&value);
                Ok(Self::Sha256 { value: buf })
            }
            DtlsFingerprintAlgorithm::Sha384 => {
                if value.len() != 48 {
                    return Err(ConversionError::InvalidFingerprintLength(48, value.len()));
                }
                let mut buf = [0u8; 48];
                buf.copy_from_slice(&value);
                Ok(Self::Sha384 { value: buf })
            }
            DtlsFingerprintAlgorithm::Sha512 => {
                if value.len() != 64 {
                    return Err(ConversionError::InvalidFingerprintLength(64, value.len()));
                }
                let mut buf = [0u8; 64];
                buf.copy_from_slice(&value);
                Ok(Self::Sha512 { value: buf })
            }
            DtlsFingerprintAlgorithm::Unspecified => Err(ConversionError::InvalidDtlsAlgorithm),
        }
    }
}

impl TryFrom<MediasoupDtlsFingerprint> for DtlsFingerprint {
    type Error = ConversionError;

    fn try_from(fingerprint: MediasoupDtlsFingerprint) -> Result<Self, Self::Error> {
        match fingerprint {
            MediasoupDtlsFingerprint::Sha1 { value } => Ok(Self {
                algorithm: DtlsFingerprintAlgorithm::Sha1 as i32,
                value: value.to_vec(),
            }),
            MediasoupDtlsFingerprint::Sha224 { value } => Ok(Self {
                algorithm: DtlsFingerprintAlgorithm::Sha224 as i32,
                value: value.to_vec(),
            }),
            MediasoupDtlsFingerprint::Sha256 { value } => Ok(Self {
                algorithm: DtlsFingerprintAlgorithm::Sha256 as i32,
                value: value.to_vec(),
            }),
            MediasoupDtlsFingerprint::Sha384 { value } => Ok(Self {
                algorithm: DtlsFingerprintAlgorithm::Sha384 as i32,
                value: value.to_vec(),
            }),
            MediasoupDtlsFingerprint::Sha512 { value } => Ok(Self {
                algorithm: DtlsFingerprintAlgorithm::Sha512 as i32,
                value: value.to_vec(),
            }),
        }
    }
}

impl TryFrom<DtlsParameters> for MediasoupDtlsParameters {
    type Error = ConversionError;

    fn try_from(params: DtlsParameters) -> Result<Self, Self::Error> {
        let role = DtlsRole::try_from(params.role)
            .map_err(|_| ConversionError::InvalidDtlsRole(params.role))?
            .try_into()
            .map_err(|_| ConversionError::InvalidDtlsRole(params.role))?;

        let fingerprints = params
            .fingerprints
            .into_iter()
            .map(MediasoupDtlsFingerprint::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            role,
            fingerprints,
        })
    }
}

impl TryFrom<MediasoupDtlsParameters> for DtlsParameters {
    type Error = ConversionError;

    fn try_from(params: MediasoupDtlsParameters) -> Result<Self, Self::Error> {
        let role = DtlsRole::from(params.role) as i32;

        let fingerprints = params
            .fingerprints
            .into_iter()
            .map(DtlsFingerprint::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            role,
            fingerprints,
        })
    }
}

impl From<MediasoupTransportId> for TransportId {
    fn from(id: MediasoupTransportId) -> Self {
        Self { id: id.to_string() }
    }
}
